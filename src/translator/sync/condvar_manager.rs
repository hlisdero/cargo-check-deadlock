//! Central structure to keep track of the condition variables in the code.
//!
//! The `CondvarManager` stores the condition variables discovered so far in the code.
//! It also performs the translation for each condition variable function.

use super::condvar::Condvar;
use super::MutexManager;
use crate::error_handling::handle_err_add_arc;
use crate::naming::condvar::{new_transition_labels, wait_transition_labels};
use crate::translator::mir_function::Memory;
use crate::translator::special_function::call_foreign_function;
use crate::utils::{
    extract_first_argument_for_function_call, extract_second_argument_for_function_call,
};
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

    /// Translates a call to `std::sync::Condvar::wait` using
    /// a representation specific to this function.
    /// A separate counter is incremented every time that
    /// the function is called to generate a unique label.
    /// Returns the pair of transitions that represent the function call.
    pub fn translate_call_wait(
        &mut self,
        start_place: &PlaceRef,
        end_place: &PlaceRef,
        net: &mut PetriNet,
    ) -> (TransitionRef, TransitionRef) {
        let index = self.wait_counter;
        self.wait_counter += 1;
        Self::create_wait_function_call(start_place, end_place, &wait_transition_labels(index), net)
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

    /// Translates the side effects for `std::sync::Condvar::wait` i.e.,
    /// the specific logic of waiting on a condition variable.
    /// Receives a reference to the memory of the caller function to retrieve the lock guard
    /// contained in the local variable for the call.
    pub fn translate_side_effects_wait<'tcx>(
        &self,
        args: &[rustc_middle::mir::Operand<'tcx>],
        wait_transitions: &(TransitionRef, TransitionRef),
        net: &mut PetriNet,
        mutex_manager: &mut MutexManager,
        memory: &mut Memory<'tcx>,
    ) {
        // Retrieve the lock guard from the local variable passed to the function as an argument.
        let lock_guard = extract_second_argument_for_function_call(args);
        let mutex_ref = memory.get_linked_lock_guard(&lock_guard);
        // Unlock the mutex when waiting, lock it when the waiting ends.
        mutex_manager.add_unlock_guard(mutex_ref, &wait_transitions.0, net);
        mutex_manager.add_lock_guard(mutex_ref, &wait_transitions.1, net);
        // Retrieve the condvar from the local variable passed to the function as an argument.
        let self_ref = extract_first_argument_for_function_call(args);
        let condvar_ref = memory.get_linked_condvar(&self_ref);
        self.link_to_wait_call(condvar_ref, wait_transitions, net);
    }

    /// Adds a new condition variable and creates its corresponding representation in the Petri net.
    /// Returns a reference to the new condition variable.
    fn add_condvar(&mut self, net: &mut PetriNet) -> CondvarRef {
        let index = self.condvars.len();
        self.condvars.push(Condvar::new(index, net));
        CondvarRef(index)
    }

    /// Creates a new representation for a call to `std::sync::Condvar::wait`.
    /// - Start place connected to a new "wait start" transition.
    /// - End place connected to a new "wait end" transition.
    /// Returns the pair of two transitions.
    fn create_wait_function_call(
        start_place: &PlaceRef,
        end_place: &PlaceRef,
        transition_labels: &(String, String),
        net: &mut PetriNet,
    ) -> (TransitionRef, TransitionRef) {
        let wait_start_transition = net.add_transition(&transition_labels.0);
        net.add_arc_place_transition(start_place, &wait_start_transition)
            .unwrap_or_else(|_| {
                handle_err_add_arc("wait call start place", "wait start transition");
            });

        let wait_end_transition = net.add_transition(&transition_labels.1);
        net.add_arc_transition_place(&wait_end_transition, end_place)
            .unwrap_or_else(|_| {
                handle_err_add_arc("wait end transition", "wait call end place");
            });

        (wait_start_transition, wait_end_transition)
    }

    /// Links the condition variable to the representation of
    /// a call to `std::sync::Condvar::wait`.
    fn link_to_wait_call(
        &self,
        condvar_ref: &CondvarRef,
        wait_transitions: &(TransitionRef, TransitionRef),
        net: &mut PetriNet,
    ) {
        let condvar = self.get_condvar_from_ref(condvar_ref);
        condvar.link_to_wait_call(&wait_transitions.0, &wait_transitions.1, net);
    }

    /// Get the condition variable corresponding to the condvar reference.
    ///
    /// # Panics
    ///
    /// If the condvar reference is invalid, then the function panics.
    fn get_condvar_from_ref(&self, condvar_ref: &CondvarRef) -> &Condvar {
        self.condvars
            .get(condvar_ref.0)
            .expect("BUG: The condvar reference should be a valid index for the vector of condition variables")
    }
}
