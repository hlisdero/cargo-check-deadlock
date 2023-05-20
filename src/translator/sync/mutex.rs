//! Representation of a mutex and a mutex guard in the Petri net.
//!
//! The mutex stores one reference to the place in the Petri net
//! that models the state of the mutex. It also stores a vector
//! of transitions where the value of the mutex was set.
//! This enables modelling the condition in a while loop for a condition variable.
//! Every value assigned after dereferencing a mutex guard associated to the mutex
//! will be treated as if setting a boolean flag for a condition variable.
//!
//! If the place has a token, the mutex is unlocked.
//! If the place does not have a token, the mutex is locked.
//!
//! A mutex guard simply contains a reference to the corresponding mutex.

use log::debug;
use std::cell::RefCell;
use std::rc::Rc;

use super::MutexRef;

use crate::data_structures::petri_net_interface::{
    add_arc_place_transition, add_arc_transition_place,
};
use crate::data_structures::petri_net_interface::{PetriNet, PlaceRef, TransitionRef};
use crate::naming::function::foreign_call_transition_labels;
use crate::naming::mutex::{condition_label, place_label};
use crate::translator::function::Places;
use crate::translator::mir_function::Memory;
use crate::translator::special_function::call_foreign_function;
use crate::utils::extract_nth_argument_as_place;

#[derive(PartialEq, Eq)]
pub struct Mutex {
    mutex: PlaceRef,
    deref_mut: RefCell<Vec<TransitionRef>>,
}

impl Mutex {
    /// Creates a new mutex whose label is based on `index`.
    /// Adds a place to the Petri Net.
    pub fn new(index: usize, net: &mut PetriNet) -> Self {
        let label = place_label(index);
        let mutex = net.add_place(&label);
        net.add_token(&mutex, 1)
            .expect("BUG: Adding initial token to mutex place should not cause an overflow");

        Self {
            mutex,
            deref_mut: RefCell::new(Vec::new()),
        }
    }

    /// Adds a lock arc for this mutex.
    /// Connects the mutex's place to the transition, then the transition will only
    /// fire if the mutex is unlocked.
    pub fn add_lock_arc(&self, lock_transition: &TransitionRef, net: &mut PetriNet) {
        add_arc_place_transition(net, &self.mutex, lock_transition);
    }

    /// Adds an unlock arc for this mutex.
    /// Connects the transition to the mutex's place, then the transition will
    /// replenish the token in the mutex when it fires.
    pub fn add_unlock_arc(&self, unlock_transition: &TransitionRef, net: &mut PetriNet) {
        add_arc_transition_place(net, unlock_transition, &self.mutex);
    }

    /// Adds a transition of a call to `std::ops::DerefMut::deref_mut`.
    /// This transition has set a value for a mutex and must be used to disable the condition variable later.
    pub fn add_deref_mut_transition(&self, transition: TransitionRef) {
        self.deref_mut.borrow_mut().push(transition);
    }

    /// Creates a new place that models the value of the condition used for a condition variable.
    /// Connects the given place through each transition that was added to the mutex to this new condition place.
    /// This represents setting a value for the condition at every function call saved before.
    /// Returns the place for the mutex condition.
    pub fn connect_set_condition(
        &self,
        index: usize,
        place_ref: &PlaceRef,
        net: &mut PetriNet,
    ) -> Option<PlaceRef> {
        if self.deref_mut.borrow().is_empty() {
            debug!("NO TRANSITIONS TO SET THE CONDITION");
            return None; // The mutex value was never set in the code
        }
        let condition = net.add_place(&condition_label(index));
        for transition_ref in self.deref_mut.borrow().iter() {
            debug!(
                "SET CONDITION WITH TRANSITION {}",
                transition_ref.to_string()
            );
            add_arc_place_transition(net, place_ref, transition_ref);
            add_arc_transition_place(net, transition_ref, &condition);
        }
        Some(condition)
    }
}

#[derive(PartialEq, Eq)]
pub struct Guard {
    pub mutex: MutexRef,
}

impl Guard {
    /// Creates a new mutex guard for a given mutex reference.
    /// By default, it is not set.
    pub const fn new(mutex: MutexRef) -> Self {
        Self { mutex }
    }
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
    let lock_transition = transitions.get_default();

    // Retrieve the mutex from the local variable passed to the function as an argument.
    let self_ref = extract_nth_argument_as_place(args, 0).unwrap_or_else(|| {
        panic!("BUG: `{function_name}` should receive the self reference as a place")
    });
    let mutex_ref = memory.mutex.get_linked_value(&self_ref);
    mutex_ref.add_lock_arc(lock_transition, net);

    // Create a new mutex guard
    let mutex_guard = Rc::new(Guard::new(Rc::clone(mutex_ref)));

    // The return value contains a new mutex guard. Link the local variable to it.
    memory.mutex_guard.link_place(destination, mutex_guard);
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
    memory.mutex.link_place(destination, mutex);
    debug!("NEW MUTEX: {destination:?}");
}

/// Checks whether the variable to be dropped is a mutex guard.
/// If that is the case, adds an unlock arc for the mutex corresponding to the mutex guard.
/// The unlock arc is added for the usual transition as well as the cleanup transition.
/// Otherwise do nothing.
pub fn handle_mutex_guard_drop<'tcx>(
    place: rustc_middle::mir::Place<'tcx>,
    unlock_transition: &TransitionRef,
    net: &mut PetriNet,
    memory: &mut Memory<'tcx>,
) {
    if memory.mutex_guard.is_linked(&place) {
        let mutex_guard_ref = memory.mutex_guard.get_linked_value(&place);
        mutex_guard_ref.mutex.add_unlock_arc(unlock_transition, net);
        debug!("DROP MUTEX GUARD {place:?} DUE TO TRANSITION {unlock_transition}");
    }
}
