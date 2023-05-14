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
use crate::utils::{check_substring_in_place_type, extract_nth_argument_as_place};

pub use condvar::Condvar;
pub use condvar_manager::{CondvarManager, CondvarRef};
use log::debug;
pub use mutex_manager::{MutexManager, MutexRef};
pub use thread::Thread;
pub use thread_manager::{ThreadManager, ThreadRef};

const SUPPORTED_SYNC_FUNCTIONS: [&str; 8] = [
    "std::mem::drop",
    "std::sync::Condvar::new",
    "std::sync::Condvar::notify_one",
    "std::sync::Condvar::wait",
    "std::sync::Mutex::<T>::lock",
    "std::sync::Mutex::<T>::new",
    "std::thread::spawn",
    "std::thread::JoinHandle::<T>::join",
];

/// Checks whether the function name corresponds to one of the
/// supported synchronization or multithreading functions.
pub fn is_supported_sync_function(function_name: &str) -> bool {
    for name in SUPPORTED_SYNC_FUNCTIONS {
        if function_name == name {
            return true;
        }
    }
    false
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
///
/// Receives a reference to the memory of the caller function to
/// link the return local variable to the synchronization variable.
///
/// This handler works for MIR assignments of the form:
/// - `_X = _Y`
/// - `_X = &_Y`
/// - `_X = move _Y`
/// - `_X = (*_Y).Z:`
/// - `_X = &((*_Y).Z)`
/// - `_X = move (*_Y).Z`
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
    if place_linked.is_indirect() {
        // Create a new place without the projections
        let mut base_place = *place_linked;
        base_place.projection = rustc_middle::ty::List::empty();
        debug!("SEARCH FOR SYNC VARIABLE IN BASE PLACE: {base_place:?} <- {place_linked:?}");

        // In the indirect case the place linked to the sync variable
        // is actually the base place of `place_linked`.
        generalized_link_place_if_sync_variable(
            place_to_be_linked,
            &base_place,
            place_linked,
            memory,
            caller_function_def_id,
            tcx,
        );
    } else {
        // In the normal case the place linked to the sync variable
        // is simply `place_linked`.
        generalized_link_place_if_sync_variable(
            place_to_be_linked,
            place_linked,
            place_linked,
            memory,
            caller_function_def_id,
            tcx,
        );
    }
}

/// Checks if `place_to_check_type` contains a mutex, a lock guard, a join handle or a condition variable.
/// If `place_to_check_type` is of type of a synchronization variable, links `place_linked` to `place_to_be_linked`.
///
/// This function decouples the place with the type of the sync variable from the place that is linked to the sync
/// variable. In this sense, it is "generalized" from the naive idea that these two concepts always match.
fn generalized_link_place_if_sync_variable<'tcx>(
    place_to_be_linked: &rustc_middle::mir::Place<'tcx>,
    place_linked: &rustc_middle::mir::Place<'tcx>,
    place_to_check_type: &rustc_middle::mir::Place<'tcx>,
    memory: &mut Memory<'tcx>,
    caller_function_def_id: rustc_hir::def_id::DefId,
    tcx: rustc_middle::ty::TyCtxt<'tcx>,
) {
    if check_substring_in_place_type(
        place_to_check_type,
        "std::sync::MutexGuard<",
        caller_function_def_id,
        tcx,
    ) {
        memory.link_place_to_same_lock_guard(*place_to_be_linked, *place_linked);
    }
    if check_substring_in_place_type(
        place_to_check_type,
        "std::sync::Mutex<",
        caller_function_def_id,
        tcx,
    ) {
        memory.link_place_to_same_mutex(*place_to_be_linked, *place_linked);
    }
    if check_substring_in_place_type(
        place_to_check_type,
        "std::thread::JoinHandle<",
        caller_function_def_id,
        tcx,
    ) {
        memory.link_place_to_same_join_handle(*place_to_be_linked, *place_linked);
    }
    if check_substring_in_place_type(
        place_to_check_type,
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
/// If there is no first argument or it is a constant,
/// then there is nothing to check, therefore the function simply returns.
///
/// Why check only the first argument?
/// Because most function in the standard library involving synchronization primitives
/// receive it through the first argument. For instance:
///  * `std::clone::Clone::clone`
///  * `std::ops::Deref::deref`
///  * `std::ops::DerefMut::deref_mut`
///  * `std::result::Result::<T, E>::unwrap`
///  * `std::sync::Arc::<T>::new`
///
/// Receives a reference to the memory of the caller function to
/// link the return local variable to the synchronization variable.
pub fn link_return_value_if_sync_variable<'tcx>(
    args: &[rustc_middle::mir::Operand<'tcx>],
    return_value: rustc_middle::mir::Place<'tcx>,
    memory: &mut Memory<'tcx>,
    caller_function_def_id: rustc_hir::def_id::DefId,
    tcx: rustc_middle::ty::TyCtxt<'tcx>,
) {
    let Some(first_argument) = extract_nth_argument_as_place(args, 0) else {
         // Nothing to check: Either the first argument is not present or it is a constant.
        return;
    };
    link_if_sync_variable(
        &return_value,
        &first_argument,
        memory,
        caller_function_def_id,
        tcx,
    );
}
