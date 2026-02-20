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

use crate::data_structures::petri_net_interface::{PetriNet, PlaceRef, TransitionRef};
use crate::data_structures::petri_net_interface::{
    add_arc_place_transition, add_arc_transition_place, connect_places,
};
use crate::naming::condvar::wait_skip_label;
use crate::naming::mutex::{condition_place_labels, place_label};
use crate::translator::function::{Places, PostprocessingTask};
use crate::translator::mir_function::memory::{Memory, MutexRef};
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

    /// Links the mutex to a condition variable.
    ///
    /// - Creates two new places `condition_not_set` and `condition_set` that model
    ///   the value of the condition used for a condition variable.
    /// - Connects `condition_not_set` through each transition saved before to `condition_set`.
    ///   This represents setting the condition at every function call discovered so far.
    /// - Connects `condition_set` and `start_place` to a new `wait_skip` transition.
    /// - Connects `wait_skip` to `end_place`.
    /// - Connects `condition_not_set` to the `wait_start` transition.
    pub fn link_to_condvar(
        &self,
        index: usize,
        start_place: &PlaceRef,
        end_place: &PlaceRef,
        wait_start: &TransitionRef,
        net: &mut PetriNet,
    ) {
        if self.deref_mut.borrow().is_empty() {
            debug!("NO TRANSITIONS TO SET THE CONDITION");
            return; // The mutex value was never set in the code
        }
        // Create condition places
        let (p1, p2) = condition_place_labels(index);
        let condition_not_set = net.add_place(&p1);
        let condition_set = net.add_place(&p2);
        net.add_token(&condition_not_set, 1).expect(
            "BUG: Adding initial token to `condition_not_set` should not cause an overflow",
        );

        for transition_ref in self.deref_mut.borrow().iter() {
            debug!("SET CONDITION WITH TRANSITION {transition_ref}");
            add_arc_place_transition(net, &condition_not_set, transition_ref);
            add_arc_transition_place(net, transition_ref, &condition_set);
        }
        // Only allow the wait if the condition is NOT set. Regenerate the token if wait is called.
        add_arc_place_transition(net, &condition_not_set, wait_start);
        add_arc_transition_place(net, wait_start, &condition_not_set);
        // Create the skip transition that short circuits the condvar logic
        let wait_skip = connect_places(net, start_place, end_place, &wait_skip_label(index));
        // Only allow to skip the wait if the condition is set. Regenerate the token if wait is called.
        add_arc_place_transition(net, &condition_set, &wait_skip);
        add_arc_transition_place(net, &wait_skip, &condition_set);
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
    args: &[rustc_span::source_map::Spanned<rustc_middle::mir::Operand<'tcx>>],
    destination: rustc_middle::mir::Place<'tcx>,
    places: Places,
    net: &mut PetriNet,
    memory: &mut Memory,
) {
    let places = places.ignore_cleanup_place();
    let transitions = call_foreign_function(function_name, index, places, net);
    let lock_transition = transitions.get_default();

    // Retrieve the mutex from the local variable passed to the function as an argument.
    let self_ref = extract_nth_argument_as_place(args, 0).unwrap_or_else(|| {
        panic!("BUG: `{function_name}` should receive the self reference as a place")
    });
    let mutex_ref = memory.get_mutex(&self_ref);
    mutex_ref.add_lock_arc(lock_transition, net);

    // Create a new mutex guard
    let mutex_guard = Guard::new(mutex_ref.clone());

    // The return value contains a new mutex guard. Link the local variable to it.
    memory.link_mutex_guard(destination, mutex_guard);
    debug!("NEW MUTEX GUARD {destination:?} DUE TO TRANSITION {lock_transition}");
}

/// Call to `std::sync::Mutex::<T>::new`.
/// Non-recursive call for the translation process.
///
/// - Creates a new `Mutex`.
/// - Links the return place to the `Mutex`.
/// - Returns a postprocessing task to notify the creation of this mutex.
pub fn call_new(
    function_name: &str,
    index: usize,
    destination: rustc_middle::mir::Place,
    places: Places,
    net: &mut PetriNet,
    memory: &mut Memory,
) -> PostprocessingTask {
    call_foreign_function(function_name, index, places, net);
    // Create a new mutex
    let mutex = Mutex::new(index, net);
    // The return value contains a new mutex. Link the local variable to it.
    let mutex_ref = memory.link_mutex(destination, mutex);
    debug!("NEW MUTEX: {destination:?}");
    // Notify the translator that a new mutex has been created
    PostprocessingTask::new_mutex(mutex_ref.clone())
}

/// Checks whether the variable to be dropped is a mutex guard.
/// If that is the case, adds an unlock arc for the mutex corresponding to the mutex guard.
/// The unlock arc is added for the usual transition as well as the cleanup transition.
/// Otherwise do nothing.
pub fn handle_mutex_guard_drop(
    place: rustc_middle::mir::Place,
    unlock_transition: &TransitionRef,
    net: &mut PetriNet,
    memory: &Memory,
) {
    if memory.is_mutex_guard(&place) {
        let mutex_guard_ref = memory.get_mutex_guard(&place);
        mutex_guard_ref.mutex.add_unlock_arc(unlock_transition, net);
        debug!("DROP MUTEX GUARD {place:?} DUE TO TRANSITION {unlock_transition}");
    }
}
