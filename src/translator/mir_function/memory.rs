//! Representation of the local memory of the function.
//!
//! The memory stores a mapping of local variables (`rustc_middle::mir::Place`)
//! with mutex references. This tracks which variables contain a mutex.
//!
//! The memory stores a mapping of local variables (`rustc_middle::mir::Place`)
//! with mutex references. This tracks which variables contain a lock guard.
//!
//! The memory stores a mapping of local variables (`rustc_middle::mir::Place`)
//! with thread references. This tracks which variables contain a join handle.
//!
//! More info:
//! <https://rustc-dev-guide.rust-lang.org/mir/index.html#mir-data-types>

use crate::translator::sync::{MutexRef, ThreadRef};
use std::collections::HashMap;

#[derive(Default)]
pub struct Memory<'tcx> {
    places_linked_to_mutexes: HashMap<rustc_middle::mir::Place<'tcx>, MutexRef>,
    places_linked_to_lock_guards: HashMap<rustc_middle::mir::Place<'tcx>, MutexRef>,
    places_linked_to_join_handles: HashMap<rustc_middle::mir::Place<'tcx>, ThreadRef>,
}

impl<'tcx> Memory<'tcx> {
    /// Creates a new memory with empty mappings.
    pub fn new() -> Self {
        Self::default()
    }

    /// Marks a place as containing a mutex.
    ///
    /// # Panics
    ///
    /// If the place is already linked to a mutex, then the function panics.
    pub fn link_place_to_mutex(
        &mut self,
        place: rustc_middle::mir::Place<'tcx>,
        mutex_ref: MutexRef,
    ) {
        if self
            .places_linked_to_mutexes
            .insert(place, mutex_ref)
            .is_some()
        {
            panic!("BUG: The place should not be already linked to a mutex")
        }
    }

    /// Marks a place as containing a lock guard for a mutex.
    ///
    /// # Panics
    ///
    /// If the place is already linked to a lock guard, then the function panics.
    pub fn link_place_to_lock_guard(
        &mut self,
        place: rustc_middle::mir::Place<'tcx>,
        mutex_ref: MutexRef,
    ) {
        if self
            .places_linked_to_lock_guards
            .insert(place, mutex_ref)
            .is_some()
        {
            panic!("BUG: The place should not be already linked to a lock guard")
        }
    }

    /// Marks a place as containing a join handle for a thread.
    ///
    /// # Panics
    ///
    /// If the place is already linked to a join handle, then the function panics.
    pub fn link_place_to_join_handle(
        &mut self,
        place: rustc_middle::mir::Place<'tcx>,
        thread_ref: ThreadRef,
    ) {
        if self
            .places_linked_to_join_handles
            .insert(place, thread_ref)
            .is_some()
        {
            panic!("BUG: The place should not be already linked to a join handle")
        }
    }

    /// Returns the mutex reference linked to the given place.
    ///
    /// # Panics
    ///
    /// If the place is not linked to a mutex, then the function panics.
    pub fn get_linked_mutex(&self, place: rustc_middle::mir::Place<'tcx>) -> &MutexRef {
        self.places_linked_to_mutexes
            .get(&place)
            .expect("BUG: The place should be linked to a mutex")
    }

    /// Returns the mutex for the lock guard linked to the given place.
    ///
    /// # Panics
    ///
    /// If the place is not linked to a lock guard, then the function panics.
    pub fn get_linked_lock_guard(&self, place: rustc_middle::mir::Place<'tcx>) -> &MutexRef {
        self.places_linked_to_lock_guards
            .get(&place)
            .expect("BUG: The place should be linked to a lock guard")
    }

    /// Returns the thread reference for the join handle linked to the given place.
    ///
    /// # Panics
    ///
    /// If the place is not linked to a join handle, then the function panics.
    pub fn get_linked_join_handle(&self, place: rustc_middle::mir::Place<'tcx>) -> &ThreadRef {
        self.places_linked_to_join_handles
            .get(&place)
            .expect("BUG: The place should be linked to a join handle")
    }

    /// Checks whether the place is linked to a lock guard.
    pub fn is_linked_to_place_guard(&self, place: rustc_middle::mir::Place<'tcx>) -> bool {
        self.places_linked_to_lock_guards.contains_key(&place)
    }

    /// Links a place to the mutex linked to another place.
    /// After this operation both places point to the same mutex, i.e.,
    /// the first place is an alias for the second place.
    ///
    /// # Panics
    ///
    /// If the place to be linked is already linked to a mutex, then the function panics.
    /// If the place linked to a mutex is not linked to a mutex, then the function panics.
    pub fn link_place_to_same_mutex(
        &mut self,
        place_to_be_linked: rustc_middle::mir::Place<'tcx>,
        place_linked_to_mutex: rustc_middle::mir::Place<'tcx>,
    ) {
        let mutex_ref = self.get_linked_mutex(place_linked_to_mutex);
        self.link_place_to_mutex(place_to_be_linked, mutex_ref.clone());
    }

    /// Links a place to the join handle linked to another place.
    /// After this operation both places point to the same join handle, i.e.,
    /// the first place is an alias for the second place.
    ///
    /// # Panics
    ///
    /// If the place to be linked is already linked to a join handle, then the function panics.
    /// If the place linked to a join handle is not linked to a join handle, then the function panics.
    pub fn link_place_to_same_join_handle(
        &mut self,
        place_to_be_linked: rustc_middle::mir::Place<'tcx>,
        place_linked_to_join_handle: rustc_middle::mir::Place<'tcx>,
    ) {
        let thread_ref = self.get_linked_join_handle(place_linked_to_join_handle);
        self.link_place_to_join_handle(place_to_be_linked, thread_ref.clone());
    }
}
