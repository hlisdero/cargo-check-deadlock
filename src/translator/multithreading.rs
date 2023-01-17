//! Submodule for implementing the translation of thread primitives.
//!
//! It defines which functions are supported and how to detect them.
//! It exposes only the necessary definitions to the outside modules.

mod thread_manager;
mod thread_span;

use crate::utils::place_to_local;
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

/// Identify MIR assignments of the form: `_X = move _Y` where:
/// - `_X` is of type `std::thread::JoinHandle<T>` and
/// - `_Y` is of type `std::thread::JoinHandle<T>`.
///
/// Link the local on the left-hand side to the join handle on the right-hand side.
pub fn identify_assign_of_local_with_join_handle(
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
        if is_join_handle_declaration(local_decl) {
            let lhs = place_to_local(place);
            return Some((lhs, rhs));
        }
    }
    None
}

/// Checks whether the type of a local declaration is `std::thread::JoinHandle<T>`,
/// where `T` is a concrete type and not a type parameter.
fn is_join_handle_declaration(local_decl: &rustc_middle::mir::LocalDecl) -> bool {
    let ty_string = local_decl.ty.to_string();
    if ty_string.starts_with("std::thread::JoinHandle<") && ty_string.ends_with('>') {
        // True if join handle with concrete type
        ty_string != "std::thread::JoinHandle<T>"
    } else {
        // Not a join handle
        false
    }
}
