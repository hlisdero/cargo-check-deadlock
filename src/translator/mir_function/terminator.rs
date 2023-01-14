//! Submodule that defines the methods for handling each of the possible terminators for a basic block.
//! <https://doc.rust-lang.org/nightly/nightly-rustc/rustc_middle/mir/enum.TerminatorKind.html>

use crate::translator::error_handling::handle_err_add_arc;
use crate::translator::mir_function::MirFunction;
use crate::translator::naming::function_return_transition_label;
use netcrab::petri_net::{PetriNet, PlaceRef, TransitionRef};

impl MirFunction {
    /// Connects the active basic block to the target basic block.
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
    /// Returns the transition that represents dropping the variable.
    ///
    /// Optionally, if an unwind block is present, connects the active basic block to the next basic
    /// block identified as the argument `unwind` of the drop terminator.
    ///
    /// # Panics
    ///
    /// If there is no active basic block set, then the function panics.
    pub fn drop(
        &mut self,
        target: rustc_middle::mir::BasicBlock,
        unwind: Option<rustc_middle::mir::BasicBlock>,
        net: &mut PetriNet,
    ) -> TransitionRef {
        let (active_block, target_block) = self.get_pair_active_block_target_block(target, net);
        let transition_drop = active_block.drop(target_block, net);

        if let Some(unwind) = unwind {
            let (active_block, unwind_block) = self.get_pair_active_block_target_block(unwind, net);
            active_block.drop_unwind(unwind_block, net);
        };
        transition_drop
    }

    /// Connects the active basic block to the next basic block identified as the argument `target`
    /// of the assert terminator.
    /// Optionally, if a cleanup block is present, connects the active basic block to the next basic
    /// block identified as the argument `cleanup` of the assert terminator.
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
