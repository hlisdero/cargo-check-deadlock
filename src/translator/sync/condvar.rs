//! Representation of a condition variable in the Petri net.
//!
//! The behavior of the condition variable is modelled using
//! four places and two transitions. The corresponding net is:
//!
//! ```dot
//! digraph condvar {
//!     wait_enabled [shape="circle" xlabel="wait_enabled" label="•"];
//!     notify [shape="circle" xlabel="notify" label=""];
//!     waiting [shape="circle" xlabel="waiting" label=""];
//!     waiting_for_lock [shape="circle" xlabel="waiting_for_lock" label=""];
//!     lost_signal [shape="box" xlabel="" label="lost_signal"];
//!     notify_received [shape="box" xlabel="" label="notify_received"];
//!     wait_enabled -> lost_signal;
//!     notify -> lost_signal;
//!     lost_signal -> wait_enabled;
//!     notify -> notify_received;
//!     waiting -> notify_received;
//!     notify_received -> waiting_for_lock;
//!     notify_received -> wait_enabled;
//! }
//! ```
//!
//! `wait_enabled` and `lost_signal` model the behavior of lost signals.
//! There is a conflict between the two transitions. If `wait()` is called before `notify()`,
//! the token in `notify` will be consumed by `lost_signal`.
//! But if `wait()` is called first, then the token from `wait_enabled` will be removed,
//! preventing the `lost_signal` from firing and ensuring that a token in `waiting_for_lock`
//! is set, which will allow the waiting thread to continue.
//!
//! This Petri net model is a slightly simplified version of the one presented in the paper
//! "Modelling Multithreaded Applications Using Petri Nets" by Kavi, Moshtaghi and Chen.
//! <https://www.researchgate.net/publication/220091454_Modeling_Multithreaded_Applications_Using_Petri_Nets>

use log::debug;
use std::rc::Rc;

use super::MutexGuardRef;

use crate::data_structures::petri_net_interface::{
    add_arc_place_transition, add_arc_transition_place, connect_places,
};
use crate::data_structures::petri_net_interface::{PetriNet, PlaceRef, TransitionRef};
use crate::naming::condvar::{place_labels, transition_labels, wait_transition_labels};
use crate::naming::function::foreign_call_transition_labels;
use crate::translator::function::Places;
use crate::translator::mir_function::Memory;
use crate::translator::special_function::call_foreign_function;
use crate::utils::extract_nth_argument_as_place;

#[derive(PartialEq, Eq)]
pub struct Condvar {
    pub wait_enabled: PlaceRef,
    notify: PlaceRef,
    waiting: PlaceRef,
    waiting_for_lock: PlaceRef,
}

impl Condvar {
    /// Creates a new condition variable whose label is based on `index`.
    /// Adds its Petri net model to the net (4 places and 2 transitions).
    pub fn new(index: usize, net: &mut PetriNet) -> Self {
        let (p1, p2, p3, p4) = place_labels(index);
        let wait_enabled = net.add_place(&p1);
        let notify = net.add_place(&p2);
        let waiting = net.add_place(&p3);
        let waiting_for_lock = net.add_place(&p4);

        net.add_token(&wait_enabled, 1)
            .expect("BUG: Adding initial token to `wait_enabled` should not cause an overflow");

        let (t1, t2) = transition_labels(index);
        let lost_signal = net.add_transition(&t1);
        let notify_received = net.add_transition(&t2);

        add_arc_place_transition(net, &wait_enabled, &lost_signal);
        add_arc_place_transition(net, &notify, &lost_signal);
        add_arc_place_transition(net, &waiting, &notify_received);
        add_arc_place_transition(net, &notify, &notify_received);

        add_arc_transition_place(net, &lost_signal, &wait_enabled);
        add_arc_transition_place(net, &notify_received, &wait_enabled);
        add_arc_transition_place(net, &notify_received, &waiting_for_lock);

        Self {
            wait_enabled,
            notify,
            waiting,
            waiting_for_lock,
        }
    }

    /// Links the Petri net model of the condition variable to the representation of
    /// a call to `std::sync::Condvar::wait`.
    /// Connects the `wait_enabled` place to the `wait_start` transition.
    /// Connects the `wait_start` transition to the `waiting` place.
    /// Connects the `waiting_for_lock` place to the `wait_end` transition.
    /// Unlocks the mutex when the waiting starts, lock it when the waiting ends.
    pub fn link_to_wait_call(
        &self,
        wait_start: &TransitionRef,
        wait_end: &TransitionRef,
        mutex_guard_ref: &MutexGuardRef,
        net: &mut PetriNet,
    ) {
        add_arc_place_transition(net, &self.wait_enabled, wait_start);
        add_arc_transition_place(net, wait_start, &self.waiting);
        add_arc_place_transition(net, &self.waiting_for_lock, wait_end);

        mutex_guard_ref.mutex.add_unlock_arc(wait_start, net);
        mutex_guard_ref.mutex.add_lock_arc(wait_end, net);
    }

    /// Links the Petri net model of the condition variable to the representation of
    /// a call to `std::sync::Condvar::notify_one`.
    /// Connects the `notify_transition` transition to the `notify` place.
    pub fn link_to_notify_one_call(&self, notify_transition: &TransitionRef, net: &mut PetriNet) {
        add_arc_transition_place(net, notify_transition, &self.notify);
    }
}

/// Call to `std::sync::Condvar::new`.
/// Non-recursive call for the translation process.
///
/// - Creates a new `Condvar`.
/// - Links the return place to the `Condvar`.
pub fn call_new<'tcx>(
    function_name: &str,
    index: usize,
    destination: rustc_middle::mir::Place<'tcx>,
    places: Places,
    net: &mut PetriNet,
    memory: &mut Memory<'tcx>,
) {
    call_foreign_function(
        places,
        &foreign_call_transition_labels(function_name, index),
        net,
    );
    // Create a new condvar
    let condvar = Rc::new(Condvar::new(index, net));
    // The return value contains a new condition variable. Link the local variable to it.
    memory.condvar.link_place(destination, condvar);
    debug!("NEW CONDVAR: {destination:?}");
}

/// Call to `std::sync::Condvar::notify_one`.
/// Non-recursive call for the translation process.
///
/// - Retrieves the condvar linked to the first argument (the self reference).
/// - Creates an arc from the transition of this function call to the `notify`
///   in the Petri net model of the condvar.
///
/// In some cases, the `std::sync::Condvar::notify_one` function contains a cleanup target.
/// This target is not called in practice but creates trouble for lost signal detection.
/// The reason is that any call may fail, which is equivalent to saying that the `notify_one`
/// was never present in the program, leading to a false lost signal.
/// In conclusion: Ignore the cleanup place, do not model it. Assume `notify_one` never unwinds.
pub fn call_notify_one<'tcx>(
    function_name: &str,
    index: usize,
    args: &[rustc_middle::mir::Operand<'tcx>],
    places: Places,
    net: &mut PetriNet,
    memory: &mut Memory<'tcx>,
) {
    let places = places.ignore_cleanup_place();
    let transitions = call_foreign_function(
        places,
        &foreign_call_transition_labels(function_name, index),
        net,
    );
    // Retrieve the condvar from the local variable passed to the function as an argument.
    let self_ref = extract_nth_argument_as_place(args, 0).unwrap_or_else(|| {
        panic!("BUG: `{function_name}` should receive the self reference as a place")
    });
    let condvar_ref = memory.condvar.get_linked_value(&self_ref);
    condvar_ref.link_to_notify_one_call(transitions.get_default(), net);
}

/// Call to `std::sync::Condvar::wait`.
/// Non-recursive call for the translation process.
///
/// - Retrieves the mutex guard linked to the second argument.
/// - Retrieves the condvar linked to the first argument (the self reference).
/// - Adds the arc for the unlocking of the mutex at the start of the `wait`.
/// - Adds the arc for the locking of the mutex at the end of the `wait`.
/// - Links the return place to the mutex guard.
///
/// In some cases, the `std::sync::Condvar::wait` function contains a cleanup target.
/// This target is not called in practice but creates trouble for lost signal detection.
/// The reason is that any call may fail, which is equivalent to saying that the `wait`
/// was never present in the program, leading to a false model.
/// In conclusion: Ignore the cleanup place, do not model it. Assume `wait` never unwinds.
pub fn call_wait<'tcx>(
    function_name: &str,
    index: usize,
    args: &[rustc_middle::mir::Operand<'tcx>],
    destination: rustc_middle::mir::Place<'tcx>,
    places: Places,
    net: &mut PetriNet,
    memory: &mut Memory<'tcx>,
) {
    let places = places.ignore_cleanup_place();
    let transition_labels = wait_transition_labels(function_name, index);

    // Create the transitions for the call
    let (start_place, end_place) = places.get_start_end_place();
    let wait_start = net.add_transition(&transition_labels.0);
    add_arc_place_transition(net, &start_place, &wait_start);
    let wait_end = net.add_transition(&transition_labels.1);
    add_arc_transition_place(net, &wait_end, &end_place);

    // Retrieve the mutex guard from the local variable passed to the function as an argument.
    let mutex_guard = extract_nth_argument_as_place(args, 1).unwrap_or_else(|| {
        panic!("BUG: `{function_name}` should receive the first argument as a place")
    });
    let mutex_guard_ref = memory.mutex_guard.get_linked_value(&mutex_guard);

    // Retrieve the condvar from the local variable passed to the function as an argument.
    let self_ref = extract_nth_argument_as_place(args, 0).unwrap_or_else(|| {
        panic!("BUG: `{function_name}` should receive the self reference as a place")
    });
    let condvar_ref = memory.condvar.get_linked_value(&self_ref);
    // Connect the transitions to the condition variable
    condvar_ref.link_to_wait_call(&wait_start, &wait_end, mutex_guard_ref, net);

    // Disable the condition variable by connecting it to every transition that set the value of the mutex.
    let condition =
        mutex_guard_ref
            .mutex
            .connect_set_condition(index, &condvar_ref.wait_enabled, net);
    if let Some(condition) = condition {
        // Create the skip transition that short circuits the whole condvar logic
        let wait_skip = connect_places(net, &start_place, &end_place, &transition_labels.2);
        // Only allow to skip the wait if the condition is set
        add_arc_place_transition(net, &condition, &wait_skip);
    }

    // The return value contains the mutex guard passed to the function. Link the local variable to it.
    let cloned_ref = Rc::clone(mutex_guard_ref);
    memory.mutex_guard.link_place(destination, cloned_ref);
}
