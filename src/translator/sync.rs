//! Submodule for implementing the translation of synchronization primitives
//! and the translation of thread primitives.
//!
//! It exposes only the necessary definitions to the outside modules.

mod arc_manager;
mod condvar;
mod condvar_manager;
mod mutex;
mod mutex_manager;
mod thread;
mod thread_manager;

use crate::translator::mir_function::Memory;
use crate::utils::is_place_with_concrete_type;

pub use arc_manager::ArcManager;
pub use condvar::Condvar;
pub use condvar_manager::{CondvarManager, CondvarRef};
pub use mutex_manager::{MutexManager, MutexRef};
pub use thread::Thread;
pub use thread_manager::{ThreadManager, ThreadRef};

/// Handles MIR assignments of sync primitives of the form: `_X = _Y`.
/// If the right-hand side contains a synchronization variable, link it to the left-hand side.
pub fn handle_use_copy_assignment<'tcx>(
    place: &rustc_middle::mir::Place<'tcx>,
    rhs: &rustc_middle::mir::Place<'tcx>,
    memory: &mut Memory<'tcx>,
    caller_function_def_id: rustc_hir::def_id::DefId,
    tcx: rustc_middle::ty::TyCtxt<'tcx>,
) {
    if is_place_with_concrete_type(rhs, "&std::sync::Mutex<T>", caller_function_def_id, tcx) {
        memory.link_place_to_same_mutex(*place, *rhs);
    } else if is_place_with_concrete_type(
        rhs,
        "&std::sync::MutexGuard<'a, T>",
        caller_function_def_id,
        tcx,
    ) {
        memory.link_place_to_same_lock_guard(*place, *rhs);
    } else if is_place_with_concrete_type(
        rhs,
        "&std::thread::JoinHandle<T>",
        caller_function_def_id,
        tcx,
    ) {
        memory.link_place_to_same_join_handle(*place, *rhs);
    } else if is_place_with_concrete_type(rhs, "&std::sync::Condvar", caller_function_def_id, tcx) {
        memory.link_place_to_same_condvar(*place, *rhs);
    }
}

/// Handles MIR assignments of sync primitives of the form: `_X = move _Y`.
/// If the right-hand side contains a synchronization variable, link it to the left-hand side.
pub fn handle_use_move_assignment<'tcx>(
    place: &rustc_middle::mir::Place<'tcx>,
    rhs: &rustc_middle::mir::Place<'tcx>,
    memory: &mut Memory<'tcx>,
    caller_function_def_id: rustc_hir::def_id::DefId,
    tcx: rustc_middle::ty::TyCtxt<'tcx>,
) {
    if is_place_with_concrete_type(rhs, "std::sync::Mutex<T>", caller_function_def_id, tcx)
        || is_place_with_concrete_type(
            rhs,
            "std::sync::Arc<std::sync::Mutex<T>>",
            caller_function_def_id,
            tcx,
        )
    {
        memory.link_place_to_same_mutex(*place, *rhs);
    } else if is_place_with_concrete_type(
        rhs,
        "std::sync::MutexGuard<'a, T>",
        caller_function_def_id,
        tcx,
    ) {
        memory.link_place_to_same_lock_guard(*place, *rhs);
    } else if is_place_with_concrete_type(
        rhs,
        "std::thread::JoinHandle<T>",
        caller_function_def_id,
        tcx,
    ) {
        memory.link_place_to_same_join_handle(*place, *rhs);
    } else if is_place_with_concrete_type(rhs, "std::sync::Condvar", caller_function_def_id, tcx) {
        memory.link_place_to_same_condvar(*place, *rhs);
    }
}

/// Handles MIR assignments of the form: `_X = &_Y`.
/// If the right-hand side contains a synchronization variable, link it to the left-hand side.
pub fn handle_ref_assignment<'tcx>(
    place: &rustc_middle::mir::Place<'tcx>,
    rhs: &rustc_middle::mir::Place<'tcx>,
    memory: &mut Memory<'tcx>,
    caller_function_def_id: rustc_hir::def_id::DefId,
    tcx: rustc_middle::ty::TyCtxt<'tcx>,
) {
    if is_place_with_concrete_type(rhs, "std::sync::Mutex<T>", caller_function_def_id, tcx)
        || is_place_with_concrete_type(
            rhs,
            "std::sync::Arc<std::sync::Mutex<T>>",
            caller_function_def_id,
            tcx,
        )
    {
        memory.link_place_to_same_mutex(*place, *rhs);
    } else if is_place_with_concrete_type(
        place,
        "std::sync::MutexGuard<'a, T>",
        caller_function_def_id,
        tcx,
    ) {
        memory.link_place_to_same_lock_guard(*place, *rhs);
    } else if is_place_with_concrete_type(
        place,
        "std::thread::JoinHandle<T>",
        caller_function_def_id,
        tcx,
    ) {
        memory.link_place_to_same_join_handle(*place, *rhs);
    } else if is_place_with_concrete_type(rhs, "std::sync::Condvar", caller_function_def_id, tcx) {
        memory.link_place_to_same_condvar(*place, *rhs);
    }
}

/// Handles MIR assignments of the form: `_X = { copy_data: move _Y }`.
/// If the right-hand side contains a synchronization variable, link it to the left-hand side.
pub fn handle_aggregate_assignment<'tcx>(
    place: &rustc_middle::mir::Place<'tcx>,
    operands: &Vec<rustc_middle::mir::Operand<'tcx>>,
    memory: &mut Memory<'tcx>,
    caller_function_def_id: rustc_hir::def_id::DefId,
    tcx: rustc_middle::ty::TyCtxt<'tcx>,
) {
    for operand in operands {
        // Extract the place to be assignment
        let rhs = match operand {
            rustc_middle::mir::Operand::Copy(place) | rustc_middle::mir::Operand::Move(place) => {
                place
            }
            // Nothing to do if we found a constant as one of the operands.
            rustc_middle::mir::Operand::Constant(_) => continue,
        };

        if is_place_with_concrete_type(rhs, "std::sync::Mutex<T>", caller_function_def_id, tcx)
            || is_place_with_concrete_type(
                rhs,
                "std::sync::Arc<std::sync::Mutex<T>>",
                caller_function_def_id,
                tcx,
            )
        {
            memory.link_place_to_same_mutex(*place, *rhs);
        } else if is_place_with_concrete_type(
            rhs,
            "std::sync::MutexGuard<'a, T>",
            caller_function_def_id,
            tcx,
        ) {
            memory.link_place_to_same_lock_guard(*place, *rhs);
        } else if is_place_with_concrete_type(
            rhs,
            "std::thread::JoinHandle<T>",
            caller_function_def_id,
            tcx,
        ) {
            memory.link_place_to_same_join_handle(*place, *rhs);
        } else if is_place_with_concrete_type(
            rhs,
            "std::sync::Condvar",
            caller_function_def_id,
            tcx,
        ) {
            memory.link_place_to_same_condvar(*place, *rhs);
        }
    }
}
