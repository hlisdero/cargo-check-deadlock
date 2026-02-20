//! Representation of a condition variable in the Petri net.
//!
//! The behavior of the condition variable is modelled using
//! two places and three transitions, plus some additional places for the mutex
//! and the variable containing the condition for the condition variable.
//! The corresponding net can be found in `./assets/condition_variable_model.dot`
//! and `./assets/condition_variable_model.svg`.
//!
//! `wait_enabled` and `lost_signal` model the behavior of lost signals.
//! There is a conflict between the two transitions. If `wait()` is called before `notify()`,
//! the token in `notify` will be consumed by `lost_signal`.
//! But if `wait()` is called first, then the token from `wait_enabled` will be removed,
//! preventing `lost_signal` from firing and ensuring that an output token is set,
//! which will allow the waiting thread to continue.
//!
//! This Petri net model is a modified version of the one presented in the paper
//! "Modelling Multithreaded Applications Using Petri Nets" by Kavi, Moshtaghi and Chen.
//! <https://www.researchgate.net/publication/220091454_Modeling_Multithreaded_Applications_Using_Petri_Nets>
//! The model was extended to model the condition on which the condition variable waits
//! and unnecessary intermediate places were removed.

use log::debug;
use std::cell::OnceCell;

use crate::data_structures::petri_net_interface::{PetriNet, PlaceRef, TransitionRef};
use crate::data_structures::petri_net_interface::{
    add_arc_place_transition, add_arc_transition_place,
};
use crate::naming::condvar::{place_labels, transition_labels};
use crate::translator::function::{Places, PostprocessingTask};
use crate::translator::mir_function::memory::{Memory, MutexGuardRef};
use crate::translator::special_function::call_foreign_function;
use crate::utils::extract_nth_argument_as_place;

#[derive(PartialEq, Eq)]
pub struct Condvar {
    wait_start: TransitionRef,
    notify: PlaceRef,
    notify_received: TransitionRef,
    already_linked_to_call: OnceCell<()>,
}

impl Condvar {
    /// Creates a new condition variable whose label is based on `index`.
    /// Adds its Petri net model to the net.
    pub fn new(index: usize, net: &mut PetriNet) -> Self {
        let (p1, p2) = place_labels(index);
        let wait_enabled = net.add_place(&p1);
        let notify = net.add_place(&p2);

        net.add_token(&wait_enabled, 1)
            .expect("BUG: Adding initial token to `wait_enabled` should not cause an overflow");

        let (t1, t2, t3) = transition_labels(index);
        let wait_start = net.add_transition(&t1);
        let lost_signal = net.add_transition(&t2);
        let notify_received = net.add_transition(&t3);

        // Loop for consuming the token in `notify` when `wait()` has not been called yet.
        add_arc_place_transition(net, &wait_enabled, &lost_signal);
        add_arc_place_transition(net, &notify, &lost_signal);
        add_arc_transition_place(net, &lost_signal, &wait_enabled);
        // Start the wait only if the wait is enabled
        add_arc_place_transition(net, &wait_enabled, &wait_start);
        // Exit the wait only if the notify was received
        add_arc_place_transition(net, &notify, &notify_received);
        // Regenerate the token in `wait_enabled` when exiting the wait
        add_arc_transition_place(net, &notify_received, &wait_enabled);

        Self {
            wait_start,
            notify,
            notify_received,
            already_linked_to_call: OnceCell::new(),
        }
    }

    /// Links the Petri net model of the condition variable to the representation of
    /// a call to `std::sync::Condvar::wait`.
    /// Connects the `start_place` place to the `wait_start` transition.
    /// Connects the `notify_received` transition to the `end_place`.
    /// Unlocks the mutex when the waiting starts, lock it when the waiting ends.
    ///
    /// # Panics
    ///
    /// If this function is called more than once, then the function panics.
    pub fn link_to_wait_call(
        &self,
        start_place: &PlaceRef,
        end_place: &PlaceRef,
        mutex_guard_ref: &MutexGuardRef,
        net: &mut PetriNet,
    ) {
        if self.already_linked_to_call.get().is_some() {
            unimplemented!("Multiple calls to `wait` or `wait_while` are not supported yet");
        }
        add_arc_place_transition(net, start_place, &self.wait_start);
        add_arc_transition_place(net, &self.notify_received, end_place);

        mutex_guard_ref.mutex.add_unlock_arc(&self.wait_start, net);
        mutex_guard_ref
            .mutex
            .add_lock_arc(&self.notify_received, net);
        // Mark the condvar as already linked to call
        self.already_linked_to_call.set(()).expect(
            "BUG: The condvar was already linked to a wait call before calling `link_to_wait_call`",
        );
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
pub fn call_new(
    function_name: &str,
    index: usize,
    destination: rustc_middle::mir::Place,
    places: Places,
    net: &mut PetriNet,
    memory: &mut Memory,
) {
    call_foreign_function(function_name, index, places, net);
    // Create a new condvar
    let condvar = Condvar::new(index, net);
    // The return value contains a new condition variable. Link the local variable to it.
    memory.link_condvar(destination, condvar);
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
pub fn call_notify_one(
    function_name: &str,
    index: usize,
    args: &[rustc_span::source_map::Spanned<rustc_middle::mir::Operand>],
    places: Places,
    net: &mut PetriNet,
    memory: &Memory,
) {
    let places = places.ignore_cleanup_place();
    let transitions = call_foreign_function(function_name, index, places, net);
    // Retrieve the condvar from the local variable passed to the function as an argument.
    let self_ref = extract_nth_argument_as_place(args, 0).unwrap_or_else(|| {
        panic!("BUG: `{function_name}` should receive the self reference as a place")
    });
    let condvar_ref = memory.get_condvar(&self_ref);
    condvar_ref.link_to_notify_one_call(transitions.get_default(), net);
}

/// Call to `std::sync::Condvar::wait`.
/// Non-recursive call for the translation process.
///
/// - Retrieves the condvar linked to the first argument (the self reference).
/// - Retrieves the mutex guard linked to the second argument.
/// - Connects the start and end place to the condition variable.
/// - Adds the arc for the unlocking of the mutex at the start of the `wait`.
/// - Adds the arc for the locking of the mutex at the end of the `wait`.
/// - Links the return place to the mutex guard.
/// - Returns a postprocessing task to link the mutex to the condition variable.
///
/// In some cases, the `std::sync::Condvar::wait` function contains a cleanup target.
/// This target is not called in practice but creates trouble for lost signal detection.
/// The reason is that any call may fail, which is equivalent to saying that the `wait`
/// was never present in the program, leading to a false model.
/// In conclusion: Ignore the cleanup place, do not model it. Assume `wait` never unwinds.
pub fn call_wait<'tcx>(
    function_name: &str,
    index: usize,
    args: &[rustc_span::source_map::Spanned<rustc_middle::mir::Operand<'tcx>>],
    destination: rustc_middle::mir::Place<'tcx>,
    places: Places,
    net: &mut PetriNet,
    memory: &mut Memory,
) -> PostprocessingTask {
    // Retrieve the condvar from the local variable passed to the function as an argument.
    let self_ref = extract_nth_argument_as_place(args, 0).unwrap_or_else(|| {
        panic!("BUG: `{function_name}` should receive the self reference as a place")
    });
    let condvar_ref = memory.get_condvar(&self_ref);
    // Retrieve the mutex guard from the local variable passed to the function as an argument.
    let mutex_guard = extract_nth_argument_as_place(args, 1).unwrap_or_else(|| {
        panic!("BUG: `{function_name}` should receive the first argument as a place")
    });
    let mutex_guard_ref = memory.get_mutex_guard(&mutex_guard);

    // Connect the start and end place to the condition variable
    let places = places.ignore_cleanup_place();
    let (start_place, end_place) = places.get_start_end_place();
    condvar_ref.link_to_wait_call(&start_place, &end_place, mutex_guard_ref, net);
    let wait_start = condvar_ref.wait_start.clone();

    // The return value contains the mutex guard passed to the function. Link the local variable to it.
    memory.link_place_to_same_value(destination, mutex_guard);

    // Create a postprocessing task to link the mutex to the condvar.
    // This creates the condition and skip logic.
    PostprocessingTask::link_mutex_to_condvar(index, start_place, end_place, wait_start)
}
