//! Submodule for implementing the translation of synchronization primitives
//! and the translation of thread primitives.
//!
//! It exposes only the necessary definitions to the outside modules.

mod condvar;
mod condvar_manager;
mod mutex;
mod mutex_manager;
mod thread;
mod thread_manager;

use crate::translator::mir_function::Memory;
use crate::utils::{check_substring_in_place_type, extract_nth_argument};

pub use condvar::Condvar;
pub use condvar_manager::{CondvarManager, CondvarRef};
pub use mutex_manager::{MutexManager, MutexRef};
pub use thread::Thread;
pub use thread_manager::{ThreadManager, ThreadRef};

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
        // Extract the place to be assigned
        let rhs = match operand {
            rustc_middle::mir::Operand::Copy(place) | rustc_middle::mir::Operand::Move(place) => {
                place
            }
            // Nothing to do if we found a constant as one of the operands.
            rustc_middle::mir::Operand::Constant(_) => continue,
        };
        link_if_sync_variable(place, rhs, memory, caller_function_def_id, tcx);
    }
}

/// Checks if `place_linked` contains a mutex, a lock guard, a join handle or a condition variable.
/// If `place_linked` contains a synchronization variable, links it to `place_to_be_linked`.
/// Receives a reference to the memory of the caller function to
/// link the return local variable to the synchronization variable.
///
/// This handler works for MIR assignments of the form:
/// - `_X = _Y`
/// - `_X = &_Y`
/// - `_X = move _Y`
///
/// It also works for checking if a function argument is a sync variable
/// and then linking the return value to the argument.
pub fn link_if_sync_variable<'tcx>(
    place_to_be_linked: &rustc_middle::mir::Place<'tcx>,
    place_linked: &rustc_middle::mir::Place<'tcx>,
    memory: &mut Memory<'tcx>,
    caller_function_def_id: rustc_hir::def_id::DefId,
    tcx: rustc_middle::ty::TyCtxt<'tcx>,
) {
    if check_substring_in_place_type(
        place_linked,
        "std::sync::MutexGuard<",
        caller_function_def_id,
        tcx,
    ) {
        memory.link_place_to_same_lock_guard(*place_to_be_linked, *place_linked);
    }
    if check_substring_in_place_type(
        place_linked,
        "std::sync::Mutex<",
        caller_function_def_id,
        tcx,
    ) {
        memory.link_place_to_same_mutex(*place_to_be_linked, *place_linked);
    }
    if check_substring_in_place_type(
        place_linked,
        "std::thread::JoinHandle<",
        caller_function_def_id,
        tcx,
    ) {
        memory.link_place_to_same_join_handle(*place_to_be_linked, *place_linked);
    }
    if check_substring_in_place_type(
        place_linked,
        "std::sync::Condvar",
        caller_function_def_id,
        tcx,
    ) {
        memory.link_place_to_same_condvar(*place_to_be_linked, *place_linked);
    }
}

/// Checks if the first argument for a function call contains a mutex, a lock guard,
/// a join handle or a condition variable, i.e. a synchronization variable.
/// If the first argument contains a synchronization variable, links it to the return value.
/// Receives a reference to the memory of the caller function to
/// link the return local variable to the synchronization variable.
pub fn link_return_value_if_sync_variable<'tcx>(
    args: &[rustc_middle::mir::Operand<'tcx>],
    return_value: rustc_middle::mir::Place<'tcx>,
    memory: &mut Memory<'tcx>,
    caller_function_def_id: rustc_hir::def_id::DefId,
    tcx: rustc_middle::ty::TyCtxt<'tcx>,
) {
    let first_argument = extract_nth_argument(args, 0);
    link_if_sync_variable(
        &return_value,
        &first_argument,
        memory,
        caller_function_def_id,
        tcx,
    );
}
