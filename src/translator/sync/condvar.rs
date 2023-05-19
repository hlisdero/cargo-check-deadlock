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

use log::debug;
use std::rc::Rc;

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
/// - Creates an arc from the transition of this function call to the `signal_input`
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
    let self_ref = extract_nth_argument_as_place(args, 0).expect(
        "BUG: `std::sync::Condvar::notify_one` should receive the self reference as a place",
    );
    let condvar_ref = memory.condvar.get_linked_value(&self_ref);
    condvar_ref.link_to_notify_one_call(transitions.get_transition(), net);
}

/// Call to `std::sync::Condvar::wait`.
/// Non-recursive call for the translation process.
///
/// - Retrieves the mutex guard linked to the second argument.
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
    index: usize,
    args: &[rustc_middle::mir::Operand<'tcx>],
    destination: rustc_middle::mir::Place<'tcx>,
    places: Places,
    net: &mut PetriNet,
    memory: &mut Memory<'tcx>,
) {
    let places = places.ignore_cleanup_place();
    let transition_labels = wait_transition_labels(index);

    let wait_transitions = translate_call_wait(places, &transition_labels, net);

    // Retrieve the mutex guard from the local variable passed to the function as an argument.
    let mutex_guard = extract_nth_argument_as_place(args, 1)
        .expect("BUG: `std::sync::Condvar::wait` should receive the first argument as a place");
    let mutex_guard_ref = memory.mutex_guard.get_linked_value(&mutex_guard);

    // Unlock the mutex when waiting, lock it when the waiting ends.
    mutex_guard_ref
        .mutex
        .add_unlock_arc(&wait_transitions.0, net);
    mutex_guard_ref.mutex.add_lock_arc(&wait_transitions.1, net);

    // Retrieve the condvar from the local variable passed to the function as an argument.
    let self_ref = extract_nth_argument_as_place(args, 0)
        .expect("BUG: `std::sync::Condvar::wait` should receive the self reference as a place");
    let condvar_ref = memory.condvar.get_linked_value(&self_ref);
    condvar_ref.link_to_wait_call(&wait_transitions.0, &wait_transitions.1, net);

    // The return value contains the mutex guard passed to the function. Link the local variable to it.
    let cloned_ref = Rc::clone(mutex_guard_ref);
    memory.mutex_guard.link_place(destination, cloned_ref);
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
