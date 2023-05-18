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
use crate::naming::mutex::place_label;
use crate::translator::mir_function::Memory;
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

/// Translates the side effects for `std::sync::Mutex::<T>::new` i.e.,
/// the specific logic of creating a new mutex.
/// Receives a reference to the memory of the caller function to
/// link the return local variable to the new mutex.
pub fn translate_side_effects_new<'tcx>(
    index: usize,
    return_value: rustc_middle::mir::Place<'tcx>,
    net: &mut PetriNet,
    memory: &mut Memory<'tcx>,
) {
    let mutex = Rc::new(Mutex::new(index, net));
    // The return value contains a new mutex. Link the local variable to it.
    memory.link_place_to_mutex(return_value, &mutex);
    debug!("NEW MUTEX: {return_value:?}");
}

/// Translates the side effects for `std::sync::Mutex::<T>::lock` i.e.,
/// the specific logic of locking a mutex.
/// Receives a reference to the memory of the caller function to retrieve the mutex contained
/// in the local variable for the call and to link the return local variable to the new mutex guard.
pub fn translate_side_effects_lock<'tcx>(
    args: &[rustc_middle::mir::Operand<'tcx>],
    return_value: rustc_middle::mir::Place<'tcx>,
    lock_transition: &TransitionRef,
    net: &mut PetriNet,
    memory: &mut Memory<'tcx>,
) {
    // Retrieve the mutex from the local variable passed to the function as an argument.
    let self_ref = extract_nth_argument_as_place(args, 0)
        .expect("BUG: `std::sync::Mutex::<T>::lock` should receive the self reference as a place");
    let mutex_ref = memory.get_linked_mutex(&self_ref);
    mutex_ref.add_lock_arc(lock_transition, net);

    // Create a new mutex guard
    let mutex_guard = Rc::new(Guard::new(Rc::clone(mutex_ref)));
    // The return value contains a new mutex guard. Link the local variable to it.
    memory.link_place_to_mutex_guard(return_value, &mutex_guard);
    debug!("NEW MUTEX GUARD {return_value:?} DUE TO TRANSITION {lock_transition}");
}
