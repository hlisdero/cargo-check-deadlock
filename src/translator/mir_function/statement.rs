//! Representation of a statement of a MIR function in the Petri net.
//! For an introduction to MIR see:
//! <https://rustc-dev-guide.rust-lang.org/mir/index.html>
//!
//! Each statement is modelled as a transition between two places.
//!
//! The start place is either the start place of the function or the end
//! place of the previous statement.
//! The transition points to the start place of the next statement or
//! the end of the current basic block.

use crate::naming::statement::{end_place_label, transition_label};
use crate::petri_net_interface::{add_arc_place_transition, add_arc_transition_place};
use crate::petri_net_interface::{PetriNet, PlaceRef, TransitionRef};

pub struct Statement {
    transition: TransitionRef,
    function_name: String,
    block_index: usize,
    statement_index: usize,
}

impl Statement {
    /// Creates a new statement and adds its representation to the Petri net.
    pub fn new(
        function_name: &str,
        block_index: usize,
        statement_index: usize,
        _statement: &rustc_middle::mir::Statement,
        start_place: &PlaceRef,
        net: &mut PetriNet,
    ) -> Self {
        let label = transition_label(function_name, block_index, statement_index);
        let transition = net.add_transition(&label);
        add_arc_place_transition(net, start_place, &transition);

        Self {
            transition,
            function_name: function_name.to_string(),
            block_index,
            statement_index,
        }
    }

    /// Creates an end place for this statement, connects the statement transition to it and returns it.
    /// The caller is responsible for storing this new place.
    pub fn create_end_place(&self, net: &mut PetriNet) -> PlaceRef {
        let place = net.add_place(&end_place_label(
            &self.function_name,
            self.block_index,
            self.statement_index,
        ));
        add_arc_transition_place(net, &self.transition, &place);
        place
    }

    /// Connects the statement transition to the given end place.
    pub fn connect_to_end_place(&self, end_place: &PlaceRef, net: &mut PetriNet) {
        add_arc_transition_place(net, &self.transition, end_place);
    }
}
