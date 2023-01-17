//! Submodule for implementing the translation of synchronization primitives.
//!
//! It defines which functions are supported and how to detect them.
//! It exposes only the necessary definitions to the outside modules.

mod mutex;
mod mutex_manager;

use crate::utils::place_to_local;
pub use mutex_manager::{MutexManager, MutexRef};

/// Checks whether the function name corresponds to `std::sync::Mutex::<T>::new`.
#[inline]
pub fn is_mutex_new(function_name: &str) -> bool {
    function_name == "std::sync::Mutex::<T>::new"
}

/// Checks whether the function name corresponds to `std::sync::Mutex::<T>::lock`.
#[inline]
pub fn is_mutex_lock(function_name: &str) -> bool {
    function_name == "std::sync::Mutex::<T>::lock"
}

/// Identify MIR assignments of the form: `_X = &_Y` where:
/// - `_X` is of type `&std::sync::Mutex<T>` and
/// - `_Y` is of type `std::sync::Mutex<T>`.
///
/// Link the local on the left-hand side to the mutex on the right-hand side.
pub fn identify_assign_of_local_with_mutex(
    place: &rustc_middle::mir::Place,
    rvalue: &rustc_middle::mir::Rvalue,
    body: &rustc_middle::mir::Body,
) -> Option<(rustc_middle::mir::Local, rustc_middle::mir::Local)> {
    if let rustc_middle::mir::Rvalue::Ref(_, _, rhs) = rvalue {
        // The right hand side must be a local variable with no projections.
        let Some(rhs) = rhs.as_local() else {
            return None;
        };
        let local_decl = &body.local_decls[rhs];
        if is_mutex_declaration(local_decl) {
            let lhs = place_to_local(place);
            return Some((lhs, rhs));
        }
    }
    None
}

/// Checks whether the type of a local declaration is `std::sync::Mutex<T>`,
/// where `T` is a concrete type and not a type parameter.
fn is_mutex_declaration(local_decl: &rustc_middle::mir::LocalDecl) -> bool {
    let ty_string = local_decl.ty.to_string();
    if ty_string.starts_with("std::sync::Mutex<") && ty_string.ends_with('>') {
        // True if mutex with concrete type
        ty_string != "std::sync::Mutex<T>"
    } else {
        // Not a mutex
        false
    }
}
