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

use crate::translator::error_handling::handle_err_add_arc;
use crate::translator::naming::{statement_end_place_label, statement_transition_label};
use netcrab::petri_net::{PetriNet, PlaceRef, TransitionRef};

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
        let label = statement_transition_label(function_name, block_index, statement_index);
        let transition = net.add_transition(&label);
        net.add_arc_place_transition(start_place, &transition)
            .unwrap_or_else(|_| handle_err_add_arc("start place", "statement transition"));

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
        let place = net.add_place(&statement_end_place_label(
            &self.function_name,
            self.block_index,
            self.statement_index,
        ));
        net.add_arc_transition_place(&self.transition, &place)
            .unwrap_or_else(|_| handle_err_add_arc("statement transition", "end place"));
        place
    }

    /// Connects the statement transition to the given end place.
    pub fn connect_to_end_place(&self, end_place: &PlaceRef, net: &mut PetriNet) {
        net.add_arc_transition_place(&self.transition, end_place)
            .unwrap_or_else(|_| handle_err_add_arc("statement transition", "given end place"));
    }
}
