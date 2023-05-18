//! Representation of a mutex and a mutex guard in the Petri net.
//!
//! The mutex stores one reference to the place in the Petri net
//! that models the state of the mutex.
//!
//! If the place has a token, the mutex is unlocked.
//! If the place does not have a token, the mutex is locked.
//!
//! A mutex guard contains a reference to the corresponding mutex
//! and a flag to keep track of when the value of the variable
//! inside the mutex has been set.

use super::MutexRef;
use crate::data_structures::petri_net_interface::{
    add_arc_place_transition, add_arc_transition_place,
};
use crate::data_structures::petri_net_interface::{PetriNet, PlaceRef, TransitionRef};
use crate::naming::function::foreign_call_transition_labels;
use crate::naming::mutex::place_label;
use crate::translator::function::Places;
use crate::translator::mir_function::Memory;
use crate::translator::special_function::call_foreign_function;
use crate::utils::extract_nth_argument_as_place;
use log::debug;
use std::rc::Rc;

#[derive(PartialEq, Eq)]
pub struct Mutex {
    place_ref: PlaceRef,
}

impl Mutex {
    /// Creates a new mutex whose label is based on `index`.
    /// Adds a place to the Petri Net.
    pub fn new(index: usize, net: &mut PetriNet) -> Self {
        let label = place_label(index);
        let place_ref = net.add_place(&label);
        net.add_token(&place_ref, 1)
            .expect("BUG: Adding initial token to mutex place should not cause an overflow");
        Self { place_ref }
    }

    /// Adds a lock arc for this mutex.
    /// Connects the mutex's place to the transition, then the transition will only
    /// fire if the mutex is unlocked.
    pub fn add_lock_arc(&self, lock_transition: &TransitionRef, net: &mut PetriNet) {
        add_arc_place_transition(net, &self.place_ref, lock_transition);
    }

    /// Adds an unlock arc for this mutex.
    /// Connects the transition to the mutex's place, then the transition will
    /// replenish the token in the mutex when it fires.
    pub fn add_unlock_arc(&self, unlock_transition: &TransitionRef, net: &mut PetriNet) {
        add_arc_transition_place(net, unlock_transition, &self.place_ref);
    }
}

#[derive(PartialEq, Eq)]
pub struct Guard {
    pub mutex: MutexRef,
    // is_set: bool,
}

impl Guard {
    /// Creates a new mutex guard for a given mutex reference.
    /// By default, it is not set.
    pub const fn new(mutex: MutexRef) -> Self {
        Self {
            mutex,
            // is_set: false,
        }
    }

    // Marks that the value of the mutex has been set, i.e., modified at least once.
    // pub fn set(&mut self) {
    //     self.is_set = true;
    // }
}

/// Call to `std::sync::Mutex::<T>::lock`.
/// Non-recursive call for the translation process.
///
/// - Retrieves the mutex linked to the first argument (the self reference).
/// - Adds an arc from the place of the mutex to the transition of this function call.
/// - Creates a new `MutexGuard`.
/// - Links the return place to the `MutexGuard`.
///
/// In some cases, the `std::sync::Mutex::<T>::lock` function contains a cleanup target.
/// This target is not called in practice but creates trouble for deadlock detection.
/// For instance, a simple double lock deadlock is not detected
/// because the second call could take the unwind path.
/// In conclusion: Ignore the cleanup place, do not model it. Assume `lock` never unwinds.
pub fn call_lock<'tcx>(
    function_name: &str,
    index: usize,
    args: &[rustc_middle::mir::Operand<'tcx>],
    destination: rustc_middle::mir::Place<'tcx>,
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
    let lock_transition = transitions.get_transition();

    // Retrieve the mutex from the local variable passed to the function as an argument.
    let self_ref = extract_nth_argument_as_place(args, 0)
        .expect("BUG: `std::sync::Mutex::<T>::lock` should receive the self reference as a place");
    let mutex_ref = memory.get_linked_mutex(&self_ref);
    mutex_ref.add_lock_arc(lock_transition, net);

    // Create a new mutex guard
    let mutex_guard = Rc::new(Guard::new(Rc::clone(mutex_ref)));

    // The return value contains a new mutex guard. Link the local variable to it.
    memory.link_place_to_mutex_guard(destination, &mutex_guard);
    debug!("NEW MUTEX GUARD {destination:?} DUE TO TRANSITION {lock_transition}");
}

/// Call to `std::sync::Mutex::<T>::new`.
/// Non-recursive call for the translation process.
///
/// - Creates a new `Mutex`.
/// - Links the return place to the `Mutex`.
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
    // Create a new mutex
    let mutex = Rc::new(Mutex::new(index, net));
    // The return value contains a new mutex. Link the local variable to it.
    memory.link_place_to_mutex(destination, &mutex);
    debug!("NEW MUTEX: {destination:?}");
}
