//! Central structure to keep track of the condition variables in the code.
//!
//! The `CondvarManager` stores the condition variables discovered so far in the code.
//! It also performs the translation for each condition variable function.

use super::condvar::Condvar;
use crate::data_structures::petri_net_interface::{
    add_arc_place_transition, add_arc_transition_place, connect_places,
};
use crate::data_structures::petri_net_interface::{PetriNet, TransitionRef};
use crate::naming::condvar::wait_transition_labels;
use crate::translator::function::Places;
use crate::translator::mir_function::Memory;
use crate::utils::extract_nth_argument_as_place;
use log::debug;
use std::rc::Rc;

#[derive(Default)]
pub struct CondvarManager {
    condvars: Vec<Rc<Condvar>>,
    wait_counter: usize,
}

/// A condvar reference is just a shared pointer to a cell containing the condition variable.
pub type CondvarRef = Rc<Condvar>;

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
        places: Places,
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
                add_arc_place_transition(net, &start_place, &wait_start_transition);

                let wait_end_transition = net.add_transition(&transition_labels.1);
                add_arc_transition_place(net, &wait_end_transition, &end_place);

                (wait_start_transition, wait_end_transition)
            }
            Places::WithCleanup {
                start_place,
                end_place,
                cleanup_place,
            } => {
                let wait_start_transition = net.add_transition(&transition_labels.0);
                add_arc_place_transition(net, &start_place, &wait_start_transition);

                let wait_end_transition = net.add_transition(&transition_labels.1);
                add_arc_transition_place(net, &wait_end_transition, &end_place);

                connect_places(net, &start_place, &cleanup_place, &transition_labels.2);

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
        memory.link_place_to_condvar(return_value, &condvar_ref);
        debug!("NEW CONDVAR: {return_value:?}");
    }

    /// Translates the side effects for `std::sync::Condvar::wait` i.e.,
    /// the specific logic of waiting on a condition variable.
    /// Receives a reference to the memory of the caller function to retrieve the mutex guard
    /// contained in the local variable for the call.
    pub fn translate_side_effects_wait<'tcx>(
        args: &[rustc_middle::mir::Operand<'tcx>],
        return_value: rustc_middle::mir::Place<'tcx>,
        wait_transitions: &(TransitionRef, TransitionRef),
        net: &mut PetriNet,
        memory: &mut Memory<'tcx>,
    ) {
        // Retrieve the mutex guard from the local variable passed to the function as an argument.
        let mutex_guard = extract_nth_argument_as_place(args, 1)
            .expect("BUG: `std::sync::Condvar::wait` should receive the first argument as a place");
        let mutex_guard_ref = memory.get_linked_mutex_guard(&mutex_guard);
        // Unlock the mutex when waiting, lock it when the waiting ends.
        mutex_guard_ref
            .mutex
            .add_unlock_arc(&wait_transitions.0, net);
        mutex_guard_ref.mutex.add_lock_arc(&wait_transitions.1, net);
        // Retrieve the condvar from the local variable passed to the function as an argument.
        let self_ref = extract_nth_argument_as_place(args, 0)
            .expect("BUG: `std::sync::Condvar::wait` should receive the self reference as a place");
        let condvar_ref = memory.get_linked_condvar(&self_ref);
        condvar_ref.link_to_wait_call(&wait_transitions.0, &wait_transitions.1, net);
        // The return value contains the mutex guard passed to the function. Link the local variable to it.
        memory.link_place_to_mutex_guard(return_value, &Rc::clone(mutex_guard_ref));
    }

    /// Translates the side effects for `std::sync::Condvar::notify_one` i.e.,
    /// the specific logic of notifying a thread waiting on a condition variable.
    /// Receives a reference to the memory of the caller function to retrieve the condition variable
    /// contained in the local variable for the call.
    pub fn translate_side_effects_notify_one<'tcx>(
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
        condvar_ref.link_to_notify_one_call(notify_one_transition, net);
    }

    /// Adds a new condition variable and creates its corresponding representation in the Petri net.
    /// Returns a reference to the new condition variable.
    fn add_condvar(&mut self, net: &mut PetriNet) -> CondvarRef {
        let index = self.condvars.len();
        let condvar = Rc::new(Condvar::new(index, net));
        let condvar_ref = Rc::clone(&condvar);
        self.condvars.push(condvar);
        condvar_ref
    }
}
