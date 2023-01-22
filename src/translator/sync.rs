//! Submodule for implementing the translation of synchronization primitives
//! and the translation of thread primitives.
//!
//! It exposes only the necessary definitions to the outside modules.

mod arc_manager;
mod mutex;
mod mutex_manager;
mod thread_manager;
mod thread_span;

pub use arc_manager::ArcManager;
pub use mutex_manager::{MutexManager, MutexRef};
pub use thread_manager::{ThreadManager, ThreadRef};
pub use thread_span::ThreadSpan;
