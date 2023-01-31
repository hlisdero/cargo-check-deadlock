//! Central structure to keep track of the condition variables in the code.
//!
//! The `CondvarManager` stores the condition variables discovered so far in the code.
//! It also performs the translation for each condition variable function.

use super::condvar::Condvar;
use crate::naming::condvar::new_transition_labels;
use crate::translator::mir_function::Memory;
use crate::translator::special_function::call_foreign_function;
use netcrab::petri_net::{PetriNet, PlaceRef, TransitionRef};

#[derive(Default)]
pub struct CondvarManager {
    condvars: Vec<Condvar>,
    wait_counter: usize,
}

/// A wrapper type around the indexes to the elements in `Vec<Condvar>`.
#[derive(Clone)]
pub struct CondvarRef(usize);

impl CondvarManager {
    /// Returns a new empty `CondvarManager`.
    pub fn new() -> Self {
        Self::default()
    }

    /// Translates a call to `std::sync::Condvar::new` using
    /// the same representation as in `foreign_function_call`.
    /// The labelling follows the numbering of the labels of the condition variables.
    /// Returns the transition that represents the function call.
    pub fn translate_call_new(
        &self,
        start_place: &PlaceRef,
        end_place: &PlaceRef,
        cleanup_place: Option<PlaceRef>,
        net: &mut PetriNet,
    ) -> TransitionRef {
        let index = self.condvars.len();
        call_foreign_function(
            start_place,
            end_place,
            cleanup_place,
            &new_transition_labels(index),
            net,
        )
    }

    /// Translates the side effects for `std::sync::Condvar::new` i.e.,
    /// the specific logic of creating a new condition variable.
    /// Receives a reference to the memory of the caller function to
    /// link the return local variable to the new condition variable.
    pub fn translate_side_effects_new<'tcx>(
        &mut self,
        return_value: rustc_middle::mir::Place<'tcx>,
        net: &mut PetriNet,
        memory: &mut Memory<'tcx>,
    ) {
        let condvar_ref = self.add_condvar(net);
        // The return value contains a new condition variable. Link the local variable to it.
        memory.link_place_to_condvar(return_value, condvar_ref);
    }

    /// Adds a new condition variable and creates its corresponding representation in the Petri net.
    /// Returns a reference to the new condition variable.
    fn add_condvar(&mut self, net: &mut PetriNet) -> CondvarRef {
        let index = self.condvars.len();
        self.condvars.push(Condvar::new(index, net));
        CondvarRef(index)
    }
}
