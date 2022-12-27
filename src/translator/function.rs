//! Representation of a function in the Petri net.
//!
//! The `Function` stores one reference to the start and end place in the Petri net.
//! The function is modelled a single transition.
//! Any mutexes will be connected to this transition to model the locking and unlocking behavior.
use crate::translator::local::Local;
use crate::translator::naming::function_transition_label_from_function_name;
use netcrab::petri_net::{PetriNet, PlaceRef, TransitionRef};

pub struct Function {
    pub def_id: rustc_hir::def_id::DefId,
    pub name: String,
    pub return_value: Local,
    pub args: Vec<Local>,
    pub start_place: PlaceRef,
    pub transition: TransitionRef,
    pub end_place: PlaceRef,
}

impl Function {
    pub fn new(
        def_id: rustc_hir::def_id::DefId,
        name: String,
        return_value: Local,
        args: Vec<Local>,
        start_place: PlaceRef,
        end_place: PlaceRef,
        net: &mut PetriNet,
    ) -> Self {
        // Create a transition that represents the function call and connect through it the start and end places.
        let transition = net.add_transition(&function_transition_label_from_function_name(&name));
        net.add_arc_place_transition(&start_place, &transition)
            .expect(
            "BUG: Adding an arc from the function start place to its transition should not fail",
        );
        net.add_arc_transition_place(&transition, &end_place)
            .expect(
                "BUG: Adding an arc from the function transition to its end place should not fail",
            );

        Self {
            def_id,
            name,
            return_value,
            args,
            start_place,
            transition,
            end_place,
        }
    }
}
