//! Representation of a function in the Petri net.
mod basic_block;
mod statement;

use crate::translator::function::basic_block::BasicBlock;
use crate::translator::mutex_manager::MutexManager;
use crate::translator::naming::{
    basic_block_start_place_label_from_block_index,
    function_return_transition_label_from_function_name,
};
use netcrab::petri_net::{PetriNet, PlaceRef};
use std::collections::HashMap;

pub struct Function {
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

impl Function {
    /// Creates a new function.
    /// Uses the `rustc_middle::ty::TyCtxt` to get the MIR body and the name of the function.
    /// Searches for mutexes in the local variable declarations and adds them to the `MutexManager`.
    pub fn new(
        def_id: rustc_hir::def_id::DefId,
        start_place: PlaceRef,
        end_place: PlaceRef,
        mutex_manager: &mut MutexManager,
        net: &mut PetriNet,
        tcx: &mut rustc_middle::ty::TyCtxt,
    ) -> Self {
        let function_name = tcx.def_path_str(def_id);
        Self::search_for_mutex_in_local_variables(def_id, mutex_manager, net, tcx);

        Self {
            def_id,
            name: function_name,
            start_place,
            end_place,
            active_block: None,
            basic_blocks: HashMap::new(),
        }
    }

    // Iterates the local variable declarations in the body of the function with given `DefId`
    // searching for a variable whose type is a mutex (`std::sync::Mutex`).
    // <https://doc.rust-lang.org/nightly/nightly-rustc/rustc_middle/mir/struct.LocalDecl.html>
    fn search_for_mutex_in_local_variables(
        def_id: rustc_hir::def_id::DefId,
        mutex_manager: &mut MutexManager,
        net: &mut PetriNet,
        tcx: &mut rustc_middle::ty::TyCtxt,
    ) {
        let body = tcx.optimized_mir(def_id);
        for local_decl in body.local_decls.iter() {
            let mut type_walk = local_decl.ty.walk();
            if type_walk.next().unwrap().to_string() == "struct `std::sync::Mutex`" {
                let mut is_generic = false;
                for ty in type_walk {
                    if ty.to_string() == "type parameter `T`" {
                        is_generic = true;
                        break;
                    }
                }
                if !is_generic {
                    let _mutex_ref = mutex_manager.add_mutex(net);
                }
            }
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
            net.add_place(&basic_block_start_place_label_from_block_index(index))
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
    /// https://doc.rust-lang.org/nightly/nightly-rustc/rustc_middle/mir/struct.BasicBlock.html
    ///
    /// # Panics
    ///
    /// If the block number was already present, then the function panics.
    fn add_basic_block(&mut self, block_number: rustc_middle::mir::BasicBlock, net: &mut PetriNet) {
        // Extracts the value of this index as a usize.
        let index = block_number.index();
        let start_place = self.prepare_start_place_for_next_basic_block(index, net);
        let basic_block = BasicBlock::new(start_place, net);
        if let Some(_) = self.basic_blocks.insert(block_number, basic_block) {
            panic!("BUG: Basic blocks should only be added once to the function.")
        }
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
    pub fn activate_block<'net>(
        &mut self,
        block: rustc_middle::mir::BasicBlock,
        net: &'net mut PetriNet,
    ) {
        if !self.basic_blocks.contains_key(&block) {
            self.add_basic_block(block, net);
        };
        self.active_block = Some(block);
    }

    /// Returns the end place of the currently active basic block.
    pub fn get_active_block_end_place(&self) -> &PlaceRef {
        let active_block = self.get_active_block();
        &active_block.end_place
    }

    /// Returns the start place of block with the given block number.
    /// Adds it to the function if it is not present already.
    pub fn get_basic_block_start(
        &mut self,
        block_number: rustc_middle::mir::BasicBlock,
        net: &mut PetriNet,
    ) -> &PlaceRef {
        if !self.basic_blocks.contains_key(&block_number) {
            self.add_basic_block(block_number, net);
        }
        let block = self
            .basic_blocks
            .get(&block_number)
            .expect("BUG: The basic block cannot be retrieved");
        &block.start_place
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
        let active_block = self.get_active_block();
        active_block.finish_statement_block(net);
    }

    /// Connects the active basic block to the next basic block identified as the argument `to`
    /// of the goto terminator.
    /// <https://doc.rust-lang.org/nightly/nightly-rustc/rustc_middle/mir/enum.TerminatorKind.html#variant.Goto>
    ///
    /// # Panics
    ///
    /// If there is no active basic block set, then the function panics.
    pub fn goto(&mut self, to: rustc_middle::mir::BasicBlock, net: &mut PetriNet) {
        let (active_block, to) = self.get_pair_active_block_target_block(to, net);
        active_block.goto(to, net);
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
    pub fn unwind(&mut self, unwind_place: PlaceRef, net: &mut PetriNet) {
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

    /// Connects the active basic block to the end place of the function.
    /// This corresponds to the return statement that exits from this function.
    /// <https://doc.rust-lang.org/nightly/nightly-rustc/rustc_middle/mir/enum.TerminatorKind.html#variant.Return>
    pub fn function_return(&mut self, net: &mut PetriNet) {
        let start_place = self.prepare_start_place_for_return_statement();
        let label = function_return_transition_label_from_function_name(&self.name);

        let transition = net.add_transition(&label);
        net.add_arc_place_transition(&start_place, &transition).expect(
            "BUG: Adding an arc from the last place inside the function to the return statement transition should not fail",
        );
        net.add_arc_transition_place(&transition, &self.end_place).expect(
            "BUG: Adding an arc from the return statement transition to the end place of the function should not fail",
        );
    }
}
