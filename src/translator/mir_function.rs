//! Representation of a MIR function in the Petri net.
//! For an introduction to MIR see:
//! <https://rustc-dev-guide.rust-lang.org/mir/index.html>
//!
//! The `MirFunction` stores a vector of `BasicBlock` which are connected between them through
//! different terminator statements such as goto, switch int, call, unwind, assert, drop, etc.
//! The `MirFunction` keeps track of an active block, the one currently being translated.
//! The operations are performed mostly on this block.
//!
//! The basic blocks are indexed by the type `rustc_middle::mir::BasicBlock` in the representation of the body.
//! <https://doc.rust-lang.org/stable/nightly-rustc/rustc_middle/mir/struct.Body.html>
//! The order in which the blocks get visited during the translation is linear.
//! But the terminators may refer to blocks previously seen or new blocks. Each basic block is visited only once.

mod basic_block;
mod memory;
mod statement;
mod terminator;

use crate::petri_net_interface::{PetriNet, PlaceRef};
use crate::translator::function_call::FunctionPlaces;
use basic_block::BasicBlock;
use std::collections::HashMap;

pub use memory::{Memory, MutexEntries};

pub struct MirFunction<'tcx> {
    /// The ID that uniquely identifies the function in this crate in the HIR representation.
    /// <https://doc.rust-lang.org/stable/nightly-rustc/rustc_hir/def_id/struct.DefId.html>
    pub def_id: rustc_hir::def_id::DefId,
    /// The name of the function as a string.
    pub name: String,
    /// The start place of the function in the Petri net.
    pub start_place: PlaceRef,
    /// The end place of the function in the Petri net.
    pub end_place: PlaceRef,
    /// The index of the basic block currently being translated.
    active_block: Option<rustc_middle::mir::BasicBlock>,
    /// A mapping between the basic block number and our representation of the basic block.
    basic_blocks: HashMap<rustc_middle::mir::BasicBlock, BasicBlock>,
    /// A representation of the memory of the function.
    pub memory: Memory<'tcx>,
}

impl<'tcx> MirFunction<'tcx> {
    /// Creates a new function.
    /// Uses the `rustc_middle::ty::TyCtxt` to get the MIR body and the name of the function.
    pub fn new(
        def_id: rustc_hir::def_id::DefId,
        start_place: PlaceRef,
        end_place: PlaceRef,
        tcx: &mut rustc_middle::ty::TyCtxt,
    ) -> Self {
        let function_name = tcx.def_path_str(def_id);

        Self {
            def_id,
            name: function_name,
            start_place,
            end_place,
            active_block: None,
            basic_blocks: HashMap::new(),
            memory: Memory::new(),
        }
    }

    /// Prepares the start place for the next block.
    /// If it is the first block, then the start place of the function
    /// is the same as the start place of the block.
    /// Otherwise returns `None` and the basic block will create a start place.
    fn prepare_start_place_for_next_basic_block(&self) -> Option<PlaceRef> {
        if self.basic_blocks.is_empty() {
            Some(self.start_place.clone())
        } else {
            None
        }
    }

    /// Prepares the start place for the function return statement.
    /// If the function has no basic blocks, the last place in the chain of basic blocks
    /// is the start place of the function.
    /// Otherwise the start place is the end place of the last basic block.
    fn prepare_start_place_for_return_statement(&self) -> PlaceRef {
        if self.basic_blocks.is_empty() {
            self.start_place.clone()
        } else {
            let active_block = self.get_active_block();
            active_block.end_place.clone()
        }
    }

    /// Returns an immutable reference to the active basic block.
    ///
    /// # Panics
    ///
    /// If the active basic block is not set, then the functions panics.
    /// If the active basic block cannot be retrieved, then the function panics.
    fn get_active_block(&self) -> &BasicBlock {
        let active_block_index = self.active_block.expect(
            "BUG: Function should have an active basic block set before calling methods that modify it.",
        );
        let active_block = self.basic_blocks.get(&active_block_index).expect(
            "BUG: The basic block cannot be retrieved. The index for the active block is invalid.",
        );
        active_block
    }

    /// Returns a mutable reference to the active basic block.
    ///
    /// # Panics
    ///
    /// If the active basic block is not set, then the functions panics.
    /// If the active basic block cannot be retrieved, then the function panics.
    fn get_mut_active_block(&mut self) -> &mut BasicBlock {
        let active_block_index = self.active_block.expect(
            "BUG: Function should have an active basic block set before calling methods that modify it.",
        );
        let active_block = self.basic_blocks.get_mut(&active_block_index).expect(
            "BUG: The basic block cannot be retrieved. The index for the active block is invalid.",
        );
        active_block
    }

    /// Adds a new basic block to the function.
    /// Receives the block number (`rustc_middle::mir::BasicBlock`) which is just an index to a vector
    /// of `rustc_middle::mir::BasicBlockData` in the MIR body of the function.
    /// <https://doc.rust-lang.org/nightly/nightly-rustc/rustc_middle/mir/struct.BasicBlock.html>
    ///
    /// # Panics
    ///
    /// If the block number was already present, then the function panics.
    fn add_basic_block(&mut self, block_number: rustc_middle::mir::BasicBlock, net: &mut PetriNet) {
        // Extracts the value of this index as a usize.
        let index = block_number.index();
        let start_place = self.prepare_start_place_for_next_basic_block();
        let basic_block = BasicBlock::new(&self.name, index, start_place, net);
        if self
            .basic_blocks
            .insert(block_number, basic_block)
            .is_some()
        {
            panic!("BUG: Basic blocks should only be added once to the function.")
        }
    }

    /// Checks if the block number is already present and adds the basic block to the function
    /// if it is not already present. Returns an immutable reference to the basic block.
    /// Receives the block number (`rustc_middle::mir::BasicBlock`) which is just an index to a vector
    /// of `rustc_middle::mir::BasicBlockData` in the MIR body of the function.
    /// <https://doc.rust-lang.org/nightly/nightly-rustc/rustc_middle/mir/struct.BasicBlock.html>
    fn get_or_add_basic_block(
        &mut self,
        block_number: rustc_middle::mir::BasicBlock,
        net: &mut PetriNet,
    ) -> &BasicBlock {
        if !self.basic_blocks.contains_key(&block_number) {
            self.add_basic_block(block_number, net);
        }
        self.basic_blocks
            .get(&block_number)
            .expect("BUG: The basic block cannot be retrieved")
    }

    /// Retrieves the active basic block and the target basic block with the given basic block number.
    /// Adds the target basic block it if it is not present already.
    ///
    /// # Panics
    ///
    /// If either basic block cannot be retrieved, then the function panics.
    fn get_pair_active_block_target_block(
        &mut self,
        block_number: rustc_middle::mir::BasicBlock,
        net: &mut PetriNet,
    ) -> (&BasicBlock, &BasicBlock) {
        if !self.basic_blocks.contains_key(&block_number) {
            self.add_basic_block(block_number, net);
        }
        let target_block = self
            .basic_blocks
            .get(&block_number)
            .expect("BUG: The target basic block cannot be retrieved");
        let active_block = self.get_active_block();
        (active_block, target_block)
    }

    /// Activates the given basic block. Adds it to the function if it is not present already.
    pub fn activate_block(&mut self, block: rustc_middle::mir::BasicBlock, net: &mut PetriNet) {
        if !self.basic_blocks.contains_key(&block) {
            self.add_basic_block(block, net);
        };
        self.active_block = Some(block);
    }

    /// Returns the start place for a function call, i.e., the end place of the current active block.
    pub fn get_start_place_for_function_call(&self) -> PlaceRef {
        let active_block = self.get_active_block();
        active_block.end_place.clone()
    }

    /// Returns a 3-tuple of the form `(start_place, end_place, cleanup_place)`
    /// where:
    ///  - `start_place` is the end place of the currently active basic block.
    ///  - `end_place` is the start place of the block with the given block number,
    ///     which is added to the function if it is not present already.
    ///  - `cleanup_place` (optional) is the place for cleanups in case the function call unwinds.
    ///     Usually foreign function calls have this.
    ///
    /// Clones the references to simplify using them.
    pub fn get_place_refs_for_function_call(
        &mut self,
        block_number: rustc_middle::mir::BasicBlock,
        cleanup_block_number: Option<rustc_middle::mir::BasicBlock>,
        net: &mut PetriNet,
    ) -> FunctionPlaces {
        let active_block = self.get_active_block();
        let start_place = active_block.end_place.clone();

        let return_block = self.get_or_add_basic_block(block_number, net);
        let end_place = return_block.start_place.clone();

        let mut cleanup_place = None;
        if let Some(cleanup_block_number) = cleanup_block_number {
            let cleanup_block = self.get_or_add_basic_block(cleanup_block_number, net);
            cleanup_place = Some(cleanup_block.start_place.clone());
        }

        (start_place, end_place, cleanup_place)
    }

    /// Adds a statement to the active basic block.
    ///
    /// # Panics
    ///
    /// If there is no active basic block set, then the function panics.
    pub fn add_statement(&mut self, statement: &rustc_middle::mir::Statement, net: &mut PetriNet) {
        let active_block = self.get_mut_active_block();
        active_block.add_statement(statement, net);
    }

    /// Finishes the statement block of the active basic block.
    /// No more statements may be added to this basic block.
    ///
    /// # Panics
    ///
    /// If there is no active basic block set, then the function panics.
    pub fn finish_statement_block(&mut self, net: &mut PetriNet) {
        let active_block = self.get_mut_active_block();
        active_block.finish_statement_block(net);
    }
}
