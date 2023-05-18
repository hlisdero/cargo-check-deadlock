//! Representation of a condition variable in the Petri net.
//!
//! The behavior of the condition variable is modelled using
//! four places and two transitions. The corresponding net is:
//!
//! ```dot
//! digraph condvar {
//!     lost_signal_possible [shape="circle" xlabel="lost_signal_possible" label="•"];
//!     signal_input [shape="circle" xlabel="signal_input" label=""];
//!     wait_input [shape="circle" xlabel="wait_input" label=""];
//!     signal_output [shape="circle" xlabel="signal_output" label=""];
//!     lost_signal_transition [shape="box" xlabel="" label="lost_signal_transition"];
//!     signal_transition [shape="box" xlabel="" label="signal_transition"];
//!     lost_signal_possible -> lost_signal_transition;
//!     signal_input -> lost_signal_transition;
//!     lost_signal_transition -> lost_signal_possible;
//!     signal_input -> signal_transition;
//!     wait_input -> signal_transition;
//!     signal_transition -> signal_output;
//!     signal_transition -> lost_signal_possible;
//! }
//! ```
//!
//! `lost_signal_possible` and `lost_signal_transition` model the behavior of lost signals.
//! There is a conflict between the two transitions. If `wait()` is called before `signal()`,
//! the token in `signal_input` will be consumed by `lost_signal_transition`.
//! But if `wait()` is called first, then the token from `lost_signal_possible` will be removed,
//! preventing the `lost_signal_transition` from firing and ensuring that a token in `signal_output`
//! is set, which will allow the waiting thread to continue.
//!
//! This Petri net model is a slightly simplified version of the one presented in the paper
//! "Modelling Multithreaded Applications Using Petri Nets" by Kavi, Moshtaghi and Chen.
//! <https://www.researchgate.net/publication/220091454_Modeling_Multithreaded_Applications_Using_Petri_Nets>

use crate::data_structures::petri_net_interface::{
    add_arc_place_transition, add_arc_transition_place, connect_places,
};
use crate::data_structures::petri_net_interface::{PetriNet, PlaceRef, TransitionRef};
use crate::naming::condvar::{place_labels, transition_labels};
use crate::translator::function::Places;
use crate::translator::mir_function::Memory;
use crate::utils::extract_nth_argument_as_place;
use log::debug;
use std::rc::Rc;

#[derive(PartialEq, Eq)]
pub struct Condvar {
    lost_signal_possible: PlaceRef,
    signal_input: PlaceRef,
    wait_input: PlaceRef,
    signal_output: PlaceRef,
}

impl Condvar {
    /// Creates a new condition variable whose label is based on `index`.
    /// Adds its Petri net model to the net (4 places and 2 transitions).
    pub fn new(index: usize, net: &mut PetriNet) -> Self {
        let (p1, p2, p3, p4) = place_labels(index);
        let lost_signal_possible = net.add_place(&p1);
        let signal_input = net.add_place(&p2);
        let wait_input = net.add_place(&p3);
        let signal_output = net.add_place(&p4);

        net.add_token(&lost_signal_possible, 1).expect(
            "BUG: Adding initial token to `lost_signal_place` should not cause an overflow",
        );

        let (t1, t2) = transition_labels(index);
        let lost_signal_transition = net.add_transition(&t1);
        let signal_transition = net.add_transition(&t2);

        add_arc_place_transition(net, &lost_signal_possible, &lost_signal_transition);
        add_arc_place_transition(net, &signal_input, &lost_signal_transition);
        add_arc_place_transition(net, &wait_input, &signal_transition);
        add_arc_place_transition(net, &signal_input, &signal_transition);

        add_arc_transition_place(net, &lost_signal_transition, &lost_signal_possible);
        add_arc_transition_place(net, &signal_transition, &lost_signal_possible);
        add_arc_transition_place(net, &signal_transition, &signal_output);

        Self {
            lost_signal_possible,
            signal_input,
            wait_input,
            signal_output,
        }
    }

    /// Links the Petri net model of the condition variable to the representation of
    /// a call to `std::sync::Condvar::wait`.
    /// Connects the `lost_signal_possible` place to the `wait_start_transition` transition.
    /// Connects the `wait_start_transition` transition to the `wait_input` place.
    /// Connects the `signal_output` place to the `wait_end_transition` transition.
    pub fn link_to_wait_call(
        &self,
        wait_start_transition: &TransitionRef,
        wait_end_transition: &TransitionRef,
        net: &mut PetriNet,
    ) {
        add_arc_place_transition(net, &self.lost_signal_possible, wait_start_transition);
        add_arc_transition_place(net, wait_start_transition, &self.wait_input);
        add_arc_place_transition(net, &self.signal_output, wait_end_transition);
    }

    /// Links the Petri net model of the condition variable to the representation of
    /// a call to `std::sync::Condvar::notify_one`.
    /// Connects the `signal_transition` transition to the `signal_input` place.
    pub fn link_to_notify_one_call(&self, signal_transition: &TransitionRef, net: &mut PetriNet) {
        add_arc_transition_place(net, signal_transition, &self.signal_input);
    }
}

/// Translates a call to `std::sync::Condvar::wait` using
/// a representation specific to this function:
/// - Start place connected to a new "wait start" transition.
/// - End place connected to a new "wait end" transition.
///
/// Returns the pair of transitions that represent the function call.
pub fn translate_call_wait(
    places: Places,
    transition_labels: &(String, String, String),
    net: &mut PetriNet,
) -> (TransitionRef, TransitionRef) {
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
    index: usize,
    return_value: rustc_middle::mir::Place<'tcx>,
    net: &mut PetriNet,
    memory: &mut Memory<'tcx>,
) {
    let condvar = Rc::new(Condvar::new(index, net));
    // The return value contains a new condition variable. Link the local variable to it.
    memory.link_place_to_condvar(return_value, &condvar);
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
