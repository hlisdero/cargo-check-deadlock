//! Representation of a mutex in the Petri net.
//!
//! The `Mutex` stores one reference to the place in the Petri net
//! that models the state of the mutex.
//!
//! If the place has a token, the mutex is unlocked.
//! If the place does not have a token, the mutex is locked.
use crate::translator::error_handling::format_err_str_add_arc;
use crate::translator::naming::mutex_place_label;
use netcrab::petri_net::{PetriNet, PlaceRef, TransitionRef};

pub struct Mutex {
    place_ref: PlaceRef,
}

impl Mutex {
    /// Creates a new `Mutex` whose label is based on `index`.
    /// Adds a `Place` to the Petri Net.
    pub fn new(index: usize, net: &mut PetriNet) -> Self {
        let label = mutex_place_label(index);
        let place_ref = net.add_place(&label);
        Self { place_ref }
    }

    /// Returns `true` if the `Mutex` is locked
    ///
    /// # Panics
    ///
    /// If the stored place reference is invalid for the given Petri net, the function panics.
    /// If the place contains a number of tokens that is not 1 or 0, the function panics.
    pub fn is_locked(&self, net: &PetriNet) -> bool {
        let marking = net
            .marking(&self.place_ref)
            .expect("BUG: The mutex's place reference should be valid for the net");
        match marking {
            1 => true,
            0 => false,
            _ => panic!("BUG: The mutex's place may only have 1 or 0 tokens"),
        }
    }

    /// Adds a lock guard for this `Mutex`.
    /// Connects the mutex's place to the transition, then the transition will only
    /// fire if the mutex is unlocked.
    pub fn add_lock_guard(&self, transition_lock: &TransitionRef, net: &mut PetriNet) {
        net.add_arc_place_transition(&self.place_ref, transition_lock)
            .unwrap_or_else(|_| {
                panic!("{}", format_err_str_add_arc("mutex's place", "lock guard"))
            });
    }
}
