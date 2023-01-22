//! Submodule for implementing the translation of synchronization primitives.
//!
//! It exposes only the necessary definitions to the outside modules.

mod arc_manager;
mod mutex;
mod mutex_manager;

pub use arc_manager::ArcManager;
pub use mutex_manager::{MutexManager, MutexRef};
