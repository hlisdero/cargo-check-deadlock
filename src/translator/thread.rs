//! Submodule for implementing the translation of thread primitives.
//!
//! It defines which functions are supported and how to detect them.
//! It exposes only the necessary definitions to the outside modules.

mod thread_manager;
mod thread_span;

pub use thread_manager::ThreadManager;
pub use thread_manager::ThreadRef;
pub use thread_span::ThreadSpan;

const SUPPORTED_THREAD_FUNCTIONS: [&str; 2] =
    ["std::thread::spawn", "std::thread::JoinHandle::<T>::join"];

/// Checks whether the function name corresponds to one of the supported thread functions.
pub fn is_thread_function(function_name: &str) -> bool {
    for name in SUPPORTED_THREAD_FUNCTIONS {
        if function_name == name {
            return true;
        }
    }
    false
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
