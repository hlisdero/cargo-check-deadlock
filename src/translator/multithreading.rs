//! Submodule for implementing the translation of thread primitives.
//!
//! It defines which functions are supported and how to detect them.
//! It exposes only the necessary definitions to the outside modules.

mod thread_manager;
mod thread_span;

use crate::utils::{is_local_decl_with_concrete_type, place_to_local};

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

/// Detects MIR assignments of the form: `_X = move _Y` where:
/// - `_X` is of type `std::thread::JoinHandle<T>` and
/// - `_Y` is of type `std::thread::JoinHandle<T>`.
///
/// Returns a 2-tuple containing the left-hand side and the right-hand side.
/// Returns `None` if the assignment does not have this form.
pub fn detect_assignment_join_handle(
    place: &rustc_middle::mir::Place,
    rvalue: &rustc_middle::mir::Rvalue,
    body: &rustc_middle::mir::Body,
) -> Option<(rustc_middle::mir::Local, rustc_middle::mir::Local)> {
    if let rustc_middle::mir::Rvalue::Use(rustc_middle::mir::Operand::Move(rhs)) = rvalue {
        // The right hand side must be a local variable with no projections.
        let Some(rhs) = rhs.as_local() else {
            return None
        };
        let local_decl = &body.local_decls[rhs];
        if is_local_decl_with_concrete_type(local_decl, "std::thread::JoinHandle<T>") {
            let lhs = place_to_local(place);
            return Some((lhs, rhs));
        }
    }
    None
}
