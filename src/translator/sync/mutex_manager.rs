//! Central structure to keep track of the mutexes in the code.
//!
//! The `MutexManager` stores the mutexes discovered so far in the code.
//! It also performs the translation for each mutex function.

use super::mutex::{Guard, Mutex};
use crate::data_structures::petri_net_interface::{PetriNet, TransitionRef};
use crate::translator::mir_function::Memory;
use crate::utils::extract_nth_argument_as_place;
use log::debug;
use std::rc::Rc;

#[derive(Default)]
pub struct MutexManager {
    mutexes: Vec<Rc<Mutex>>,
    guards: Vec<Rc<Guard>>,
}

/// A mutex reference is just a shared pointer to a cell containing the mutex.
pub type MutexRef = Rc<Mutex>;

/// A mutex guard reference is just a shared pointer to a cell containing the mutex guard.
pub type MutexGuardRef = Rc<Guard>;

impl MutexManager {
    /// Returns a new empty `MutexManager`.
    pub fn new() -> Self {
        Self::default()
    }

    /// Translates the side effects for `std::sync::Mutex::<T>::new` i.e.,
    /// the specific logic of creating a new mutex.
    /// Receives a reference to the memory of the caller function to
    /// link the return local variable to the new mutex.
    pub fn translate_side_effects_new<'tcx>(
        &mut self,
        return_value: rustc_middle::mir::Place<'tcx>,
        net: &mut PetriNet,
        memory: &mut Memory<'tcx>,
    ) {
        let mutex_ref = self.add_mutex(net);
        // The return value contains a new mutex. Link the local variable to it.
        memory.link_place_to_mutex(return_value, &mutex_ref);
        debug!("NEW MUTEX: {return_value:?}");
    }

    /// Translates the side effects for `std::sync::Mutex::<T>::lock` i.e.,
    /// the specific logic of locking a mutex.
    /// Receives a reference to the memory of the caller function to retrieve the mutex contained
    /// in the local variable for the call and to link the return local variable to the new mutex guard.
    pub fn translate_side_effects_lock<'tcx>(
        &mut self,
        args: &[rustc_middle::mir::Operand<'tcx>],
        return_value: rustc_middle::mir::Place<'tcx>,
        lock_transition: &TransitionRef,
        net: &mut PetriNet,
        memory: &mut Memory<'tcx>,
    ) {
        // Retrieve the mutex from the local variable passed to the function as an argument.
        let self_ref = extract_nth_argument_as_place(args, 0).expect(
            "BUG: `std::sync::Mutex::<T>::lock` should receive the self reference as a place",
        );
        let mutex_ref = memory.get_linked_mutex(&self_ref);
        mutex_ref.add_lock_arc(lock_transition, net);

        let mutex_guard_ref = self.add_mutex_guard(mutex_ref);
        // The return value contains a new mutex guard. Link the local variable to it.
        memory.link_place_to_mutex_guard(return_value, &mutex_guard_ref);
        debug!("NEW MUTEX GUARD {return_value:?} DUE TO TRANSITION {lock_transition}");
    }

    /// Adds a new mutex and creates its corresponding representation in the Petri net.
    /// Returns a reference to the new mutex.
    fn add_mutex(&mut self, net: &mut PetriNet) -> MutexRef {
        let index = self.mutexes.len();
        let mutex = Rc::new(Mutex::new(index, net));
        let mutex_ref = Rc::clone(&mutex);
        self.mutexes.push(mutex);
        mutex_ref
    }

    /// Adds a new mutex guard from a mutex reference.
    /// Returns a reference to the new mutex guard.
    fn add_mutex_guard(&mut self, mutex_ref: &MutexRef) -> MutexGuardRef {
        // Clone the reference before storing it inside the guard.
        let mutex_ref = Rc::clone(mutex_ref);
        let mutex_guard = Rc::new(Guard::new(mutex_ref));
        let mutex_guard_ref = Rc::clone(&mutex_guard);
        self.guards.push(mutex_guard);
        mutex_guard_ref
    }
}
