//! Submodule for implementing the translation of thread primitives.
//!
//! It exposes only the necessary definitions to the outside modules.

mod thread_manager;
mod thread_span;

pub use thread_manager::ThreadManager;
pub use thread_manager::ThreadRef;
pub use thread_span::ThreadSpan;
