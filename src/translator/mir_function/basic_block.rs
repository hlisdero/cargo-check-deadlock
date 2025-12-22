//! Representation of a basic block of a MIR function in the Petri net.
//! For an introduction to MIR see:
//! <https://rustc-dev-guide.rust-lang.org/mir/index.html>
//!
//! The `BasicBlock` stores one reference to the place in the Petri net.

use crate::data_structures::petri_net_interface::{
    connect_places, PetriNet, PlaceRef, TransitionRef,
};
use crate::naming::basic_block::{
    assert_cleanup_transition_label, assert_transition_label, drop_cleanup_transition_label,
    drop_transition_label, goto_transition_label, place_label, switch_int_transition_label,
    unreachable_transition_label, unwind_transition_label,
};

pub struct BasicBlock {
    /// Name of the function to which this block belongs.
    function_name: String,
    /// Index of this block in the current function.
    index: usize,
    /// The place of this basic block in the Petri net.
    pub place: PlaceRef,
}

impl BasicBlock {
    /// Creates a new basic block and adds its representation to the Petri net.
    /// Creates a new start place for the basic block if it is not provided.
    pub fn new(
        function_name: &str,
        index: usize,
        place: Option<PlaceRef>,
        net: &mut PetriNet,
    ) -> Self {
        let place = place.unwrap_or_else(|| net.add_place(&place_label(function_name, index)));
        Self {
            function_name: function_name.to_string(),
            index,
            place,
        }
    }

    /// Connects the end place of this block to the start place of the `target` basic block.
    pub fn goto(&self, target: &Self, net: &mut PetriNet) {
        let label = goto_transition_label(&self.function_name, self.index);
        connect_places(net, &self.place, &target.place, &label);
    }

    /// Connects the end place of this block to the start place of the `target` basic block.
    pub fn switch_int(&self, target: &Self, target_index: usize, net: &mut PetriNet) {
        let label = switch_int_transition_label(&self.function_name, self.index, target_index);
        connect_places(net, &self.place, &target.place, &label);
    }

    /// Connects the end place of this block to the unwind place.
    /// Returns the new transition created to connect the basic block with the unwind place.
    pub fn unwind(&self, unwind_place: &PlaceRef, net: &mut PetriNet) -> TransitionRef {
        let label = unwind_transition_label(&self.function_name, self.index);
        connect_places(net, &self.place, unwind_place, &label)
    }

    /// Connects the end place of this block to the start place of the `target` basic block.
    /// Returns the new transition created to connect the two basic blocks.
    pub fn drop(&self, target: &Self, net: &mut PetriNet) -> TransitionRef {
        let label = drop_transition_label(&self.function_name, self.index);
        connect_places(net, &self.place, &target.place, &label)
    }

    /// Connects the end place of this block to the start place of the `cleanup` basic block.
    /// Returns the new transition created to connect the two basic blocks.
    pub fn drop_cleanup(&self, cleanup: &Self, net: &mut PetriNet) -> TransitionRef {
        let label = drop_cleanup_transition_label(&self.function_name, self.index);
        connect_places(net, &self.place, &cleanup.place, &label)
    }

    /// Connects the end place of this block to the start place of the `assert` basic block.
    pub fn assert(&self, target: &Self, net: &mut PetriNet) {
        let label = assert_transition_label(&self.function_name, self.index);
        connect_places(net, &self.place, &target.place, &label);
    }

    /// Connects the end place of this block to the start place of the `cleanup` basic block.
    pub fn assert_cleanup(&self, cleanup: &Self, net: &mut PetriNet) {
        let label = assert_cleanup_transition_label(&self.function_name, self.index);
        connect_places(net, &self.place, &cleanup.place, &label);
    }

    /// Connects the end place of this block to the end place.
    pub fn unreachable(&self, end_place: &PlaceRef, net: &mut PetriNet) {
        let label = unreachable_transition_label(&self.function_name, self.index);
        connect_places(net, &self.place, end_place, &label);
    }
}
