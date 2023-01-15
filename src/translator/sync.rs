//! Submodule for implementing the translation of synchronization primitives.
//!
//! It defines which functions are supported and how to detect them.
//! It exposes only the necessary definitions to the outside modules.

pub use memory::Memory;
pub use mutex_manager::{MutexManager, MutexRef};

mod memory;
mod mutex;
mod mutex_manager;

const SUPPORTED_MUTEX_FUNCTIONS: [&str; 2] =
    ["std::sync::Mutex::<T>::new", "std::sync::Mutex::<T>::lock"];

/// Checks whether the function name corresponds to one of the supported mutex functions.
pub fn is_mutex_function(function_name: &str) -> bool {
    for name in SUPPORTED_MUTEX_FUNCTIONS {
        if function_name == name {
            return true;
        }
    }
    false
}

/// Checks whether the type of a local declaration is `std::sync::Mutex<T>`,
/// where `T` is a concrete type and not a type parameter.
pub fn is_mutex_declaration(local_decl: &rustc_middle::mir::LocalDecl) -> bool {
    let ty_string = local_decl.ty.to_string();
    if ty_string.starts_with("std::sync::Mutex<") && ty_string.ends_with('>') {
        // True if mutex with concrete type
        ty_string != "std::sync::Mutex<T>"
    } else {
        // Not a mutex
        false
    }
}
