//! Submodule for implementing the translation of thread primitives.
//!
//! It defines which functions are supported and how to detect them.
//! It exposes only the necessary definitions to the outside modules.

mod thread_manager;
mod thread_span;

use crate::utils::{get_place_type, is_place_ty_with_concrete_type};

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
/// Returns the right-hand side place if the assignment has this form.
/// Returns `None` if the assignment does not have this form.
pub fn detect_assignment_join_handle<'tcx>(
    rvalue: &rustc_middle::mir::Rvalue<'tcx>,
    caller_function_def_id: rustc_hir::def_id::DefId,
    tcx: rustc_middle::ty::TyCtxt<'tcx>,
) -> Option<rustc_middle::mir::Place<'tcx>> {
    if let rustc_middle::mir::Rvalue::Use(rustc_middle::mir::Operand::Move(rhs)) = rvalue {
        let place_ty = get_place_type(rhs, caller_function_def_id, tcx);
        if is_place_ty_with_concrete_type(&place_ty, "std::thread::JoinHandle<T>") {
            return Some(*rhs);
        }
    }
    None
}
