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
mod statement;

use crate::translator::error_handling::handle_err_add_arc;
use crate::translator::mir_function::basic_block::BasicBlock;
use crate::translator::naming::{basic_block_start_place_label, function_return_transition_label};
use netcrab::petri_net::{PetriNet, PlaceRef};
use std::collections::HashMap;

pub struct MirFunction {
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
}

impl MirFunction {
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
        }
    }

    /// Prepares the start place for the next block.
    /// If it is the first block, then the start place of the function
    /// is the same as the start place of the block.
    /// Otherwise a new start place is created and returned.
    #[inline]
    fn prepare_start_place_for_next_basic_block(
        &self,
        index: usize,
        net: &mut PetriNet,
    ) -> PlaceRef {
        if self.basic_blocks.is_empty() {
            self.start_place.clone()
        } else {
            net.add_place(&basic_block_start_place_label(&self.name, index))
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
        let start_place = self.prepare_start_place_for_next_basic_block(index, net);
        let basic_block = BasicBlock::new(&self.name, index, start_place);
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
    ) -> (PlaceRef, PlaceRef, Option<PlaceRef>) {
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

    /// Connects the active basic block to the target basic block.
    /// <https://doc.rust-lang.org/nightly/nightly-rustc/rustc_middle/mir/enum.TerminatorKind.html#variant.Goto>
    ///
    /// # Panics
    ///
    /// If there is no active basic block set, then the function panics.
    pub fn goto(&mut self, target: rustc_middle::mir::BasicBlock, net: &mut PetriNet) {
        let (active_block, target_block) = self.get_pair_active_block_target_block(target, net);
        active_block.goto(target_block, net);
    }

    /// Connects the active basic block to all the possible basic block targets in the switch int statement.
    /// This models the execution flow taking every possible path.
    /// Adds the corresponding block if it is not present already.
    /// <https://doc.rust-lang.org/nightly/nightly-rustc/rustc_middle/mir/enum.TerminatorKind.html#variant.SwitchInt>
    ///
    /// # Panics
    ///
    /// If there is no active basic block set, then the function panics.
    pub fn switch_int(&mut self, targets: Vec<rustc_middle::mir::BasicBlock>, net: &mut PetriNet) {
        for basic_block in targets {
            let (active_block, target_block) =
                self.get_pair_active_block_target_block(basic_block, net);
            let index = basic_block.index();
            active_block.switch_int(target_block, index, net);
        }
    }

    /// Connects the active basic block to a given unwind place that models a `panic!` scenario or similar.
    ///
    /// # Panics
    ///
    /// If there is no active basic block set, then the function panics.
    pub fn unwind(&mut self, unwind_place: &PlaceRef, net: &mut PetriNet) {
        let active_block = self.get_active_block();
        active_block.unwind(unwind_place, net);
    }

    /// Connects the active basic block to the next basic block identified as the argument `target`
    /// of the drop terminator.
    /// Optionally, if an unwind block is present, connects the active basic block to the next basic
    /// block identified as the argument `unwind` of the drop terminator.
    /// <https://doc.rust-lang.org/nightly/nightly-rustc/rustc_middle/mir/enum.TerminatorKind.html#variant.Drop>
    ///
    /// # Panics
    ///
    /// If there is no active basic block set, then the function panics.
    pub fn drop(
        &mut self,
        target: rustc_middle::mir::BasicBlock,
        unwind: Option<rustc_middle::mir::BasicBlock>,
        net: &mut PetriNet,
    ) {
        let (active_block, target_block) = self.get_pair_active_block_target_block(target, net);
        active_block.drop(target_block, net);

        if let Some(unwind) = unwind {
            let (active_block, unwind_block) = self.get_pair_active_block_target_block(unwind, net);
            active_block.drop_unwind(unwind_block, net);
        };
    }

    /// Connects the active basic block to the next basic block identified as the argument `target`
    /// of the assert terminator.
    /// Optionally, if a cleanup block is present, connects the active basic block to the next basic
    /// block identified as the argument `cleanup` of the assert terminator.
    /// <https://doc.rust-lang.org/nightly/nightly-rustc/rustc_middle/mir/enum.TerminatorKind.html#variant.Assert>
    ///
    /// # Panics
    ///
    /// If there is no active basic block set, then the function panics.
    pub fn assert(
        &mut self,
        target: rustc_middle::mir::BasicBlock,
        cleanup: Option<rustc_middle::mir::BasicBlock>,
        net: &mut PetriNet,
    ) {
        let (active_block, target_block) = self.get_pair_active_block_target_block(target, net);
        active_block.assert(target_block, net);

        if let Some(cleanup) = cleanup {
            let (active_block, cleanup_block) =
                self.get_pair_active_block_target_block(cleanup, net);
            active_block.assert_cleanup(cleanup_block, net);
        };
    }

    /// Connects the active basic block to a new transition that models a "dead end" in the net.
    /// <https://doc.rust-lang.org/stable/nightly-rustc/rustc_middle/mir/enum.TerminatorKind.html#variant.Call>
    pub fn diverging_call(&self, function_name: &str, net: &mut PetriNet) {
        let active_block = self.get_active_block();
        active_block.diverging_call(function_name, net);
    }

    /// Connects the active basic block to the end place of the function.
    /// This corresponds to the return statement that exits from this function.
    /// <https://doc.rust-lang.org/nightly/nightly-rustc/rustc_middle/mir/enum.TerminatorKind.html#variant.Return>
    pub fn return_statement(&mut self, net: &mut PetriNet) {
        let start_place = self.prepare_start_place_for_return_statement();
        let label = function_return_transition_label(&self.name);

        let transition = net.add_transition(&label);
        net.add_arc_place_transition(&start_place, &transition)
            .unwrap_or_else(|_| {
                handle_err_add_arc(
                    "last place inside the function",
                    "return statement transition",
                );
            });
        net.add_arc_transition_place(&transition, &self.end_place)
            .unwrap_or_else(|_| {
                handle_err_add_arc("return statement transition", "end place of the function");
            });
    }
}
