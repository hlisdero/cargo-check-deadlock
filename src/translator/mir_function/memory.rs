//! Representation of the local memory of the function.
//!
//! The memory stores a mapping of local variables (`rustc_middle::mir::Place`)
//! with mutex references. This tracks which variables contain a mutex.
//!
//! The memory stores a mapping of local variables (`rustc_middle::mir::Place`)
//! with mutex references. This tracks which variables contain a mutex guard.
//!
//! The memory stores a mapping of local variables (`rustc_middle::mir::Place`)
//! with thread references. This tracks which variables contain a join handle.
//!
//! The memory stores a mapping of local variables (`rustc_middle::mir::Place`)
//! with condvar references. This tracks which variables contain a condition variable.
//!
//! More info:
//! <https://rustc-dev-guide.rust-lang.org/mir/index.html#mir-data-types>

use crate::data_structures::memory_map::MemoryMap;
use crate::translator::sync::{CondvarRef, MutexGuardRef, MutexRef, ThreadRef};

pub struct Memory<'tcx> {
    pub mutex: MemoryMap<'tcx, MutexRef>,
    pub mutex_guard: MemoryMap<'tcx, MutexGuardRef>,
    pub join_handle: MemoryMap<'tcx, ThreadRef>,
    pub condvar: MemoryMap<'tcx, CondvarRef>,
}

/// An auxiliary type for passing memory entries from one function to the other.
pub type Entries<T> = Vec<T>;

impl<'tcx> Memory<'tcx> {
    /// Creates a new memory with empty mappings.
    pub fn new() -> Self {
        Self {
            mutex: MemoryMap::new(),
            mutex_guard: MemoryMap::new(),
            join_handle: MemoryMap::new(),
            condvar: MemoryMap::new(),
        }
    }
}
