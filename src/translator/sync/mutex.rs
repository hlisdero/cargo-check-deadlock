//! Representation of a mutex in the Petri net.
//!
//! The mutex stores one reference to the place in the Petri net
//! that models the state of the mutex.
//!
//! If the place has a token, the mutex is unlocked.
//! If the place does not have a token, the mutex is locked.

use crate::error_handling::handle_err_add_arc;
use crate::naming::mutex_place_label;
use netcrab::petri_net::{PetriNet, PlaceRef, TransitionRef};

pub struct Mutex {
    place_ref: PlaceRef,
}

impl Mutex {
    /// Creates a new mutex whose label is based on `index`.
    /// Adds a place to the Petri Net.
    pub fn new(index: usize, net: &mut PetriNet) -> Self {
        let label = mutex_place_label(index);
        let place_ref = net.add_place(&label);
        net.add_token(&place_ref, 1)
            .expect("BUG: Adding initial token to mutex place should not cause an overflow");
        Self { place_ref }
    }

    /// Adds a lock guard for this mutex.
    /// Connects the mutex's place to the transition, then the transition will only
    /// fire if the mutex is unlocked.
    pub fn add_lock_guard(&self, transition_lock: &TransitionRef, net: &mut PetriNet) {
        net.add_arc_place_transition(&self.place_ref, transition_lock)
            .unwrap_or_else(|_| handle_err_add_arc("mutex's place", "lock guard"));
    }

    /// Adds an unlock guard for this mutex.
    /// Connects the transition to the mutex's place, then the transition will
    /// replenish the token in the mutex when it fires.
    pub fn add_unlock_guard(&self, transition_unlock: &TransitionRef, net: &mut PetriNet) {
        net.add_arc_transition_place(transition_unlock, &self.place_ref)
            .unwrap_or_else(|_| handle_err_add_arc("unlock guard", "mutex's place"));
    }
}