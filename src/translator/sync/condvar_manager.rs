//! Central structure to keep track of the condition variables in the code.
//!
//! The `CondvarManager` stores the condition variables discovered so far in the code.
//! It also performs the translation for each condition variable function.

use super::condvar::Condvar;
use super::MutexManager;
use crate::data_structures::petri_net_interface::{
    add_arc_place_transition, add_arc_transition_place, connect_places,
};
use crate::data_structures::petri_net_interface::{PetriNet, TransitionRef};
use crate::naming::condvar::wait_transition_labels;
use crate::translator::function::Places;
use crate::translator::mir_function::Memory;
use crate::utils::extract_nth_argument_as_place;
use log::debug;

#[derive(Default)]
pub struct CondvarManager {
    condvars: Vec<Condvar>,
    wait_counter: usize,
}

/// A wrapper type around the indexes to the elements in `Vec<Condvar>`.
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct CondvarRef(usize);

impl CondvarManager {
    /// Returns a new empty `CondvarManager`.
    pub fn new() -> Self {
        Self::default()
    }

    /// Translates a call to `std::sync::Condvar::wait` using
    /// a representation specific to this function:
    /// - Start place connected to a new "wait start" transition.
    /// - End place connected to a new "wait end" transition.
    ///
    /// A separate counter is incremented every time that
    /// the function is called to generate a unique label.
    ///
    /// Returns the pair of transitions that represent the function call.
    pub fn translate_call_wait(
        &mut self,
        places: &Places,
        net: &mut PetriNet,
    ) -> (TransitionRef, TransitionRef) {
        let index = self.wait_counter;
        self.wait_counter += 1;
        let transition_labels = &wait_transition_labels(index);

        match places {
            Places::Basic {
                start_place,
                end_place,
            } => {
                let wait_start_transition = net.add_transition(&transition_labels.0);
                add_arc_place_transition(net, start_place, &wait_start_transition);

                let wait_end_transition = net.add_transition(&transition_labels.1);
                add_arc_transition_place(net, &wait_end_transition, end_place);

                (wait_start_transition, wait_end_transition)
            }
            Places::WithCleanup {
                start_place,
                end_place,
                cleanup_place,
            } => {
                let wait_start_transition = net.add_transition(&transition_labels.0);
                add_arc_place_transition(net, start_place, &wait_start_transition);

                let wait_end_transition = net.add_transition(&transition_labels.1);
                add_arc_transition_place(net, &wait_end_transition, end_place);

                connect_places(net, start_place, cleanup_place, &transition_labels.2);

                (wait_start_transition, wait_end_transition)
            }
        }
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
        debug!("NEW CONDVAR: {return_value:?}");
    }

    /// Translates the side effects for `std::sync::Condvar::wait` i.e.,
    /// the specific logic of waiting on a condition variable.
    /// Receives a reference to the memory of the caller function to retrieve the lock guard
    /// contained in the local variable for the call.
    pub fn translate_side_effects_wait<'tcx>(
        &self,
        args: &[rustc_middle::mir::Operand<'tcx>],
        return_value: rustc_middle::mir::Place<'tcx>,
        wait_transitions: &(TransitionRef, TransitionRef),
        net: &mut PetriNet,
        mutex_manager: &mut MutexManager,
        memory: &mut Memory<'tcx>,
    ) {
        // Retrieve the lock guard from the local variable passed to the function as an argument.
        let lock_guard = extract_nth_argument_as_place(args, 1)
            .expect("BUG: `std::sync::Condvar::wait` should receive the first argument as a place");
        let mutex_ref = memory.get_linked_lock_guard(&lock_guard);
        // Unlock the mutex when waiting, lock it when the waiting ends.
        mutex_manager.add_unlock_guard(*mutex_ref, &wait_transitions.0, net);
        mutex_manager.add_lock_guard(*mutex_ref, &wait_transitions.1, net);
        // Retrieve the condvar from the local variable passed to the function as an argument.
        let self_ref = extract_nth_argument_as_place(args, 0)
            .expect("BUG: `std::sync::Condvar::wait` should receive the self reference as a place");
        let condvar_ref = memory.get_linked_condvar(&self_ref);
        self.link_to_wait_call(*condvar_ref, wait_transitions, net);
        // The return value contains the lock guard passed to the function. Link the local variable to it.
        memory.link_place_to_lock_guard(return_value, *mutex_ref);
    }

    /// Translates the side effects for `std::sync::Condvar::notify_one` i.e.,
    /// the specific logic of notifying a thread waiting on a condition variable.
    /// Receives a reference to the memory of the caller function to retrieve the condition variable
    /// contained in the local variable for the call.
    pub fn translate_side_effects_notify_one<'tcx>(
        &self,
        args: &[rustc_middle::mir::Operand<'tcx>],
        notify_one_transition: &TransitionRef,
        net: &mut PetriNet,
        memory: &mut Memory<'tcx>,
    ) {
        // Retrieve the condvar from the local variable passed to the function as an argument.
        let self_ref = extract_nth_argument_as_place(args, 0).expect(
            "BUG: `std::sync::Condvar::notify_one` should receive the self reference as a place",
        );
        let condvar_ref = memory.get_linked_condvar(&self_ref);
        self.link_to_notify_one_call(*condvar_ref, notify_one_transition, net);
    }

    /// Adds a new condition variable and creates its corresponding representation in the Petri net.
    /// Returns a reference to the new condition variable.
    fn add_condvar(&mut self, net: &mut PetriNet) -> CondvarRef {
        let index = self.condvars.len();
        self.condvars.push(Condvar::new(index, net));
        CondvarRef(index)
    }

    /// Links the condition variable to the representation of
    /// a call to `std::sync::Condvar::wait`.
    fn link_to_wait_call(
        &self,
        condvar_ref: CondvarRef,
        wait_transitions: &(TransitionRef, TransitionRef),
        net: &mut PetriNet,
    ) {
        let condvar = self.get_condvar_from_ref(condvar_ref);
        condvar.link_to_wait_call(&wait_transitions.0, &wait_transitions.1, net);
    }

    /// Links the condition variable to the representation of
    /// a call to `std::sync::Condvar::notify_one`.
    fn link_to_notify_one_call(
        &self,
        condvar_ref: CondvarRef,
        signal_transition: &TransitionRef,
        net: &mut PetriNet,
    ) {
        let condvar = self.get_condvar_from_ref(condvar_ref);
        condvar.link_to_notify_one_call(signal_transition, net);
    }

    /// Get the condition variable corresponding to the condvar reference.
    ///
    /// # Panics
    ///
    /// If the condvar reference is invalid, then the function panics.
    fn get_condvar_from_ref(&self, condvar_ref: CondvarRef) -> &Condvar {
        self.condvars
            .get(condvar_ref.0)
            .expect("BUG: The condvar reference should be a valid index for the vector of condition variables")
    }
}
