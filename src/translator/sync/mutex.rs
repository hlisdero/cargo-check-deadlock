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

use super::mutex_manager::MutexRef;
use crate::data_structures::petri_net_interface::{
    add_arc_place_transition, add_arc_transition_place,
};
use crate::data_structures::petri_net_interface::{PetriNet, PlaceRef, TransitionRef};
use crate::naming::mutex::place_label;

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
