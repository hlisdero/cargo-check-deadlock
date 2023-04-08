//! Submodule that defines the methods for handling each of the possible terminators for a basic block.
//! <https://doc.rust-lang.org/nightly/nightly-rustc/rustc_middle/mir/enum.TerminatorKind.html>

use super::MirFunction;
use crate::data_structures::petri_net_interface::{
    add_arc_place_transition, add_arc_transition_place,
};
use crate::data_structures::petri_net_interface::{PetriNet, PlaceRef, TransitionRef};
use crate::naming::function::return_transition_label;

impl<'tcx> MirFunction<'tcx> {
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
    /// Optionally, if the unwind action contains a cleanup block, connects the active basic block to the next basic
    /// block contained in the argument `unwind` of the drop terminator.
    ///
    /// # Panics
    ///
    /// If there is no active basic block set, then the function panics.
    pub fn drop(
        &mut self,
        target: rustc_middle::mir::BasicBlock,
        unwind: rustc_middle::mir::UnwindAction,
        net: &mut PetriNet,
    ) -> TransitionRef {
        let (active_block, target_block) = self.get_pair_active_block_target_block(target, net);
        let transition_drop = active_block.drop(target_block, net);

        if let rustc_middle::mir::UnwindAction::Cleanup(cleanup) = unwind {
            let (active_block, cleanup_block) =
                self.get_pair_active_block_target_block(cleanup, net);
            active_block.drop_cleanup(cleanup_block, net);
        };
        transition_drop
    }

    /// Connects the active basic block to the next basic block identified as the argument `target`
    /// of the assert terminator.
    /// Optionally, if the unwind action contains a cleanup block, connects the active basic block to the next basic
    /// block contained in the argument `unwind` of the assert terminator.
    ///
    /// # Panics
    ///
    /// If there is no active basic block set, then the function panics.
    pub fn assert(
        &mut self,
        target: rustc_middle::mir::BasicBlock,
        unwind: rustc_middle::mir::UnwindAction,
        net: &mut PetriNet,
    ) {
        let (active_block, target_block) = self.get_pair_active_block_target_block(target, net);
        active_block.assert(target_block, net);

        if let rustc_middle::mir::UnwindAction::Cleanup(cleanup) = unwind {
            let (active_block, cleanup_block) =
                self.get_pair_active_block_target_block(cleanup, net);
            active_block.assert_cleanup(cleanup_block, net);
        };
    }

    /// Connects the active basic block to the end place of the function.
    /// This corresponds to the return statement that exits from this function.
    pub fn return_statement(&mut self, net: &mut PetriNet) {
        let start_place = self.prepare_start_place_for_return_statement();
        let label = return_transition_label(&self.name);

        let transition = net.add_transition(&label);
        add_arc_place_transition(net, &start_place, &transition);
        add_arc_transition_place(net, &transition, &self.end_place);
    }

    /// Connects the active basic block to a given end place.
    /// We need a unique non-deadlocking end for all the terminators of this kind.
    /// <https://doc.rust-lang.org/stable/nightly-rustc/rustc_middle/mir/enum.TerminatorKind.html#variant.Unreachable>
    /// This is just the same as `unwind` but with a different label.
    ///
    /// # Panics
    ///
    /// If there is no active basic block set, then the function panics.
    pub fn unreachable(&mut self, end_place: &PlaceRef, net: &mut PetriNet) {
        let active_block = self.get_active_block();
        active_block.unreachable(end_place, net);
    }
}
