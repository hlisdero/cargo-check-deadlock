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
use crate::translator::error_handling::format_err_str_add_arc;
use crate::translator::naming::{statement_transition_label_from_statement_kind, STATEMENT_END};
use netcrab::petri_net::{PetriNet, PlaceRef, TransitionRef};

pub struct Statement {
    transition: TransitionRef,
}

impl Statement {
    /// Creates a new statement and adds its representation to the Petri net.
    pub fn new(
        statement: &rustc_middle::mir::Statement,
        start_place: &PlaceRef,
        net: &mut PetriNet,
    ) -> Self {
        let label = statement_transition_label_from_statement_kind(&statement.kind);
        let transition = net.add_transition(&label);

        net.add_arc_place_transition(start_place, &transition)
            .unwrap_or_else(|_| {
                panic!(
                    "{}",
                    format_err_str_add_arc("start place", "statement transition",)
                )
            });

        Self { transition }
    }

    /// Creates an end place for this statement, connects the statement transition to it and returns it.
    /// The caller is responsible for storing this new place.
    pub fn create_end_place(&self, net: &mut PetriNet) -> PlaceRef {
        let place = net.add_place(STATEMENT_END);
        net.add_arc_transition_place(&self.transition, &place)
            .unwrap_or_else(|_| {
                panic!(
                    "{}",
                    format_err_str_add_arc("statement transition", "end place")
                )
            });
        place
    }

    /// Connects the statement transition to the given end place.
    pub fn connect_to_end_place(&self, end_place: &PlaceRef, net: &mut PetriNet) {
        net.add_arc_transition_place(&self.transition, end_place)
            .unwrap_or_else(|_| {
                panic!(
                    "{}",
                    format_err_str_add_arc("statement transition", "given end place",)
                )
            });
    }
}
