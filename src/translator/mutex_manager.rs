//! Auxiliary structure to keep track of the mutexes in the code.
//!
//! The `MutexManager` stores the mutexes discovered so far in the code.
//! It also stores the lock guards, i.e. the local variables that result
//! from a call to `std::sync::Mutex::lock()`.
//! It also stores the links, i.e. the local variables that reference a mutex.
//! If a mutex A is accessed in two threads then two variables
//! (the original and the `.clone()` of the `std::sync::Arc`) is a link to mutex A.
use crate::translator::local::Local;
use crate::translator::mutex::Mutex;
use netcrab::petri_net::{PetriNet, TransitionRef};
use std::collections::HashMap;

#[derive(Default)]
pub struct MutexManager {
    mutexes: Vec<Mutex>,
    links: HashMap<Local, MutexRef>,
    guards: HashMap<Local, MutexRef>,
}

/// A wrapper type around the indexes to the elements in `Vec<Mutex>`.
#[derive(Clone)]
pub struct MutexRef(usize);

impl MutexManager {
    /// Return a new empty `MutexManager`
    pub fn new() -> Self {
        Self::default()
    }

    /// Return the `Mutex` associated to the mutex reference.
    ///
    /// # Errors
    ///
    /// If the `MutexRef` is invalid, then an error is returned.
    fn get_mutex_from_ref(&self, mutex_ref: &MutexRef) -> Result<&Mutex, &str> {
        if mutex_ref.0 >= self.mutexes.len() {
            return Err("The given mutex reference is invalid");
        }
        Ok(&self.mutexes[mutex_ref.0])
    }

    /// Return the `Mutex` contained in the `Local` variable.
    ///
    /// # Errors
    ///
    /// If the `Local` is not associated to a `Mutex`, then an error is returned.
    pub fn get_mutex_from_local(&self, local: &Local) -> Result<&Mutex, &str> {
        let Some(mutex_ref) = self.links.get(local) else {
            return Err("The local variable is not associated to a mutex")
        };
        self.get_mutex_from_ref(mutex_ref)
    }

    /// Add a mutex and create its corresponding representation in the Petri net.
    pub fn add_mutex(&mut self, net: &mut PetriNet) -> MutexRef {
        let index = self.mutexes.len();
        self.mutexes.push(Mutex::new(index, net));
        MutexRef(index)
    }

    /// Adds a lock guard to the mutex.
    /// Connects the mutex's place to the transition, then the transition will only
    /// fire if the mutex is unlocked.
    ///
    /// # Errors
    ///
    /// If the `Local` is not associated to a `Mutex`, then an error is returned.
    pub fn add_lock_guard(
        &mut self,
        guard: Local,
        local_with_mutex: &Local,
        transition_lock: &TransitionRef,
        net: &mut PetriNet,
    ) -> Result<(), &str> {
        let Some(mutex_ref) = self.links.get(local_with_mutex) else {
            return Err("The local variable is not associated to a mutex");
        };
        self.guards.insert(guard, mutex_ref.clone());

        let mutex = match self.get_mutex_from_ref(mutex_ref) {
            Ok(mutex) => mutex,
            Err(err_str) => return Err(err_str),
        };
        mutex.add_lock_guard(transition_lock, net);
        Ok(())
    }

    /// Check if the `Local` variable is linked to a `Mutex`.
    /// Return true if the variable contains a mutex.
    pub fn is_linked_to_mutex(&self, local: &Local) -> bool {
        self.links.contains_key(local)
    }

    /// Link the local variable to a specific mutex through its reference.
    ///
    /// If the local variable did not have an associated mutex, `None` is returned.
    /// If the local variable had an associated mutex, the value is updated and the old value is returned.
    ///
    /// This could overwrite a previous link to a different mutex.
    /// It does not occur often that one "reuses" a variable for two different mutexes
    /// but it is allowed in theory.
    pub fn link(&mut self, local: Local, mutex_ref: MutexRef) -> Option<MutexRef> {
        self.links.insert(local, mutex_ref)
    }
}
