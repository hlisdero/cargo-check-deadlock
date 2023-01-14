//! Submodule for implementing the translation of synchronization primitives.
//!
//! It exposes only the necessary definitions to the outside modules.

pub use mutex_manager::MutexManager;
pub use mutex_utils::{is_mutex_declaration, is_mutex_function};

mod mutex;
mod mutex_manager;
mod mutex_utils;
