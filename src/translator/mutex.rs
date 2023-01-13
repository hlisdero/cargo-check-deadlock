//! Submodule to define the structures and functions
//! that implement the mutex translation.
//! It exposes only the `MutexManager` to the outside.

pub use mutex_manager::MutexManager;

mod mutex;
mod mutex_manager;
