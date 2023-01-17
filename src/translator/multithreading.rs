//! Submodule for implementing the translation of thread primitives.
//!
//! It defines which functions are supported and how to detect them.
//! It exposes only the necessary definitions to the outside modules.

mod thread_manager;
mod thread_span;

pub use thread_manager::ThreadManager;
pub use thread_manager::ThreadRef;
pub use thread_span::ThreadSpan;

/// Checks whether the function name corresponds to `std::thread::spawn`.
#[inline]
pub fn is_thread_spawn(function_name: &str) -> bool {
    function_name == "std::thread::spawn"
}

/// Checks whether the function name corresponds to `std::thread::JoinHandle::<T>::join`.
#[inline]
pub fn is_thread_join(function_name: &str) -> bool {
    function_name == "std::thread::JoinHandle::<T>::join"
}

/// Checks whether the type of a local declaration is `std::thread::JoinHandle<T>`,
/// where `T` is a concrete type and not a type parameter.
pub fn is_join_handle_declaration(local_decl: &rustc_middle::mir::LocalDecl) -> bool {
    let ty_string = local_decl.ty.to_string();
    if ty_string.starts_with("std::thread::JoinHandle<") && ty_string.ends_with('>') {
        // True if join handle with concrete type
        ty_string != "std::thread::JoinHandle<T>"
    } else {
        // Not a join handle
        false
    }
}
