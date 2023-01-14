//! Representation of the local memory of the function.
//!
//! The memory stores a mapping of local variables (`rustc_middle::mir::Local`)
//! with mutex references. This tracks which variables contain a mutex.
//!
//! More info:
//! <https://rustc-dev-guide.rust-lang.org/mir/index.html#mir-data-types>

use crate::translator::sync::MutexRef;
use std::collections::HashMap;

#[derive(Default)]
pub struct Memory {
    locals_linked_to_mutexes: HashMap<rustc_middle::mir::Local, MutexRef>,
}

impl Memory {
    /// Creates a new memory with empty mappings.
    pub fn new() -> Self {
        Self::default()
    }

    /// Marks a local variable as containing a mutex.
    ///
    /// # Panics
    ///
    /// If the local variable is already linked to a mutex, then the function panics.
    pub fn link_local_to_mutex(&mut self, local: rustc_middle::mir::Local, mutex_ref: MutexRef) {
        if self
            .locals_linked_to_mutexes
            .insert(local, mutex_ref)
            .is_some()
        {
            panic!("BUG: The local should not be already linked to a mutex")
        }
    }

    /// Returns the mutex reference linked to the given local variable.
    ///
    /// # Panics
    ///
    /// If the local variable is not linked to a mutex, then the function panics.
    pub fn get_mutex_ref_from_local(&self, local: rustc_middle::mir::Local) -> &MutexRef {
        self.locals_linked_to_mutexes
            .get(&local)
            .expect("BUG: The local variable should be linked to a mutex")
    }

    /// Links a local variable to the mutex linked to another local.
    /// After this operation both locals point to the same mutex, i.e.,
    /// the first local is an alias for the second local.
    ///
    /// # Panics
    ///
    /// If the local to be linked is already linked to a mutex, then the function panics.
    /// If the local linked to a mutex is not linked to a mutex, then the function panics.
    pub fn link_local_to_same_mutex(
        &mut self,
        local_to_be_linked: rustc_middle::mir::Local,
        local_linked_to_mutex: rustc_middle::mir::Local,
    ) {
        let mutex_ref = self.get_mutex_ref_from_local(local_linked_to_mutex);
        self.link_local_to_mutex(local_to_be_linked, mutex_ref.clone());
    }
}
