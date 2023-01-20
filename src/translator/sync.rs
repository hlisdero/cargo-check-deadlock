//! Submodule for implementing the translation of synchronization primitives.
//!
//! It defines which functions are supported and how to detect them.
//! It exposes only the necessary definitions to the outside modules.

mod arc_manager;
mod mutex;
mod mutex_manager;

use crate::utils::{is_local_decl_with_concrete_type, place_to_local};

pub use arc_manager::ArcManager;
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

/// Checks whether the function name corresponds to `std::sync::Arc::<T>::new`.
#[inline]
pub fn is_arc_new(function_name: &str) -> bool {
    function_name == "std::sync::Arc::<T>::new"
}

/// Checks whether the function name corresponds to `std::ops::Deref::deref`.
#[inline]
pub fn is_deref(function_name: &str) -> bool {
    function_name == "std::ops::Deref::deref"
}

/// Detects MIR assignments of the form: `_X = &_Y` where:
/// - `_X` is of type `&std::sync::Mutex<T>` and
/// - `_Y` is of type `std::sync::Mutex<T>`.
///
/// Returns a 2-tuple containing the left-hand side and the right-hand side.
/// Returns `None` if the assignment does not have this form.
pub fn detect_assignment_reference_to_mutex(
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
        if is_local_decl_with_concrete_type(local_decl, "std::sync::Mutex<T>") {
            let lhs = place_to_local(place);
            return Some((lhs, rhs));
        }
    }
    None
}

/// Detects MIR assignments of the form: `_X = _Y` where:
/// - `_X` is of type `&std::sync::Mutex<T>` and
/// - `_Y` is of type `&std::sync::Mutex<T>`.
///
/// Returns a 2-tuple containing the left-hand side and the right-hand side.
/// Returns `None` if the assignment does not have this form.
pub fn detect_assignment_copy_reference_to_mutex(
    place: &rustc_middle::mir::Place,
    rvalue: &rustc_middle::mir::Rvalue,
    body: &rustc_middle::mir::Body,
) -> Option<(rustc_middle::mir::Local, rustc_middle::mir::Local)> {
    if let rustc_middle::mir::Rvalue::Use(rustc_middle::mir::Operand::Copy(rhs)) = rvalue {
        // The right hand side must be a local variable with no projections.
        let Some(rhs) = rhs.as_local() else {
            return None;
        };
        let local_decl = &body.local_decls[rhs];
        if is_local_decl_with_concrete_type(local_decl, "&std::sync::Mutex<T>") {
            let lhs = place_to_local(place);
            return Some((lhs, rhs));
        }
    }
    None
}

/// Detects calls to `std::sync::Arc::<T>::new` where the type of
/// the argument is `std::sync::Mutex<T>`
///
/// Returns a 2-tuple containing the return value and the argument to the function.
/// Returns `None` if the call does not have this form.
pub fn detect_mutex_inside_arc_new(
    operand: &rustc_middle::mir::Operand,
    destination: rustc_middle::mir::Place,
    body: &rustc_middle::mir::Body,
) -> Option<(rustc_middle::mir::Local, rustc_middle::mir::Local)> {
    if let rustc_middle::mir::Operand::Move(place) = operand {
        // The first argument must be a local variable with no projections.
        let Some(contained_value_local) = place.as_local() else {
            return None;
        };
        let local_decl = &body.local_decls[contained_value_local];
        if is_local_decl_with_concrete_type(local_decl, "std::sync::Mutex<T>") {
            let return_value = place_to_local(&destination);
            return Some((return_value, contained_value_local));
        }
    }
    None
}

/// Detects calls to `std::ops::Deref::deref` where the type of
/// the argument is `&std::sync::Arc<std::sync::Mutex<T>>`
///
/// Returns a 2-tuple containing the return value and the argument to the function.
/// Returns `None` if the call does not have this form.
pub fn detect_deref_arc_with_mutex(
    operand: &rustc_middle::mir::Operand,
    destination: rustc_middle::mir::Place,
    body: &rustc_middle::mir::Body,
) -> Option<(rustc_middle::mir::Local, rustc_middle::mir::Local)> {
    if let rustc_middle::mir::Operand::Move(place) = operand {
        // The first argument must be a local variable with no projections.
        let Some(contained_value_local) = place.as_local() else {
            return None;
        };
        let local_decl = &body.local_decls[contained_value_local];
        if is_local_decl_with_concrete_type(local_decl, "&std::sync::Arc<std::sync::Mutex<T>>") {
            let return_value = place_to_local(&destination);
            return Some((return_value, contained_value_local));
        }
    }
    None
}

/// Detects MIR assignments of the form: `_X = &_Y` where:
/// - `_X` is of type `&std::sync::Arc<std::sync::Mutex<T>>` and
/// - `_Y` is of type `std::sync::Arc<std::sync::Mutex<T>>`.
///
/// Returns a 2-tuple containing the left-hand side and the right-hand side.
/// Returns `None` if the assignment does not have this form.
pub fn detect_assignment_reference_to_arc_with_mutex(
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
        if is_local_decl_with_concrete_type(local_decl, "std::sync::Arc<std::sync::Mutex<T>>") {
            let lhs = place_to_local(place);
            return Some((lhs, rhs));
        }
    }
    None
}
