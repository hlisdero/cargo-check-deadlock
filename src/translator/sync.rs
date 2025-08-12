//! Submodule for implementing the translation of synchronization primitives
//! and the translation of thread primitives.

pub mod condvar;
pub mod mutex;
pub mod thread;

use log::debug;

use crate::data_structures::petri_net_interface::PetriNet;
use crate::translator::function::{Places, PostprocessingTask};
use crate::translator::mir_function::memory::Memory;
use crate::utils::{extract_nth_argument_as_place, get_type_string};

// Re-export the types that the module contains.
// It does not make assumptions about how they are stored.
// That is the responsibility of the memory.
pub use condvar::Condvar;
pub use mutex::{Guard as MutexGuard, Mutex};
pub use thread::Thread;

/// Checks whether the function name corresponds to one of the
/// supported synchronization or multithreading functions.
pub fn is_supported_function(function_name: &str) -> bool {
    matches!(
        function_name,
        "std::sync::Condvar::new"
            | "std::sync::Condvar::notify_one"
            | "std::sync::Condvar::wait"
            | "std::sync::Condvar::wait_while"
            | "std::sync::Mutex::<T>::lock"
            | "std::sync::Mutex::<T>::new"
            | "std::thread::JoinHandle::<T>::join"
    )
}

/// Calls the corresponding handler for the supported synchronization or multithreading functions.
pub fn call_function<'tcx>(
    function_name: &str,
    index: usize,
    args: &[rustc_span::source_map::Spanned<rustc_middle::mir::Operand<'tcx>>],
    destination: rustc_middle::mir::Place<'tcx>,
    places: Places,
    net: &mut PetriNet,
    memory: &mut Memory,
) -> Option<PostprocessingTask> {
    match function_name {
        "std::sync::Condvar::new" => {
            condvar::call_new(function_name, index, destination, places, net, memory);
            None
        }
        "std::sync::Condvar::notify_one" => {
            condvar::call_notify_one(function_name, index, args, places, net, memory);
            None
        }
        "std::sync::Condvar::wait" | "std::sync::Condvar::wait_while" => {
            let task =
                condvar::call_wait(function_name, index, args, destination, places, net, memory);
            Some(task)
        }
        "std::sync::Mutex::<T>::lock" => {
            mutex::call_lock(function_name, index, args, destination, places, net, memory);
            None
        }
        "std::sync::Mutex::<T>::new" => {
            let task = mutex::call_new(function_name, index, destination, places, net, memory);
            Some(task)
        }
        "std::thread::JoinHandle::<T>::join" => {
            thread::call_join(function_name, index, args, places, net, memory);
            None
        }
        _ => panic!("BUG: Call handler for {function_name} is not defined"),
    }
}

/// Checks whether a place contains a mutex variable (mutex or mutex guard)
pub fn check_if_mutex_variable<'tcx>(
    place: &rustc_middle::mir::Place<'tcx>,
    caller_function_def_id: rustc_hir::def_id::DefId,
    tcx: rustc_middle::ty::TyCtxt<'tcx>,
) -> bool {
    let ty_string = get_type_string(place, caller_function_def_id, tcx);

    ty_string.contains("std::sync::Mutex<") || ty_string.contains("std::sync::MutexGuard<")
}

/// Checks whether the LHS place contains the same type as the RHS place
/// or an aggregate which contains the same type inside.
///
/// Examples:
///
/// - lhs = `Arc<ExampleData>`, rhs = `ExampleData` -> true
/// - lhs = `ExampleData`, rhs = `Arc<ExampleData>` -> true
/// - lhs = `Condvar`, rhs = `Mutex<i32>` -> false
/// - lhs = `Mutex<i32`, rhs = `Condvar` -> false
pub fn check_aggregate_of_same_type<'tcx>(
    lhs: &rustc_middle::mir::Place<'tcx>,
    rhs: &rustc_middle::mir::Place<'tcx>,
    caller_function_def_id: rustc_hir::def_id::DefId,
    tcx: rustc_middle::ty::TyCtxt<'tcx>,
) -> bool {
    let lhs_ty = get_type_string(lhs, caller_function_def_id, tcx);
    let rhs_ty = get_type_string(rhs, caller_function_def_id, tcx);

    // Strip references because we do not care about them. For example, the following should return true
    // lhs = std::sync::Arc<std::sync::Mutex<i32>>, rhs = &std::sync::Arc<std::sync::Mutex<i32>>
    let lhs_ty = lhs_ty.strip_prefix("&").unwrap_or(&lhs_ty);
    let rhs_ty = rhs_ty.strip_prefix("&").unwrap_or(&rhs_ty);

    debug!("COMPARING: {lhs:?} WITH TYPE {lhs_ty}, {rhs:?} WITH TYPE {rhs_ty}");
    lhs_ty.contains(rhs_ty) || rhs_ty.contains(lhs_ty)
}

/// Checks if the two places should be linked to the same value.
/// This defines the main heuristic / rule for when to link:
///
/// - The types are similar, i.e. one is an aggregate of the other or viceversa
/// - The place linked appears in the memory of the current function
fn should_link_to_same_value<'tcx>(
    place_to_link: &rustc_middle::mir::Place<'tcx>,
    place_linked: &rustc_middle::mir::Place<'tcx>,
    memory: &Memory,
    caller_function_def_id: rustc_hir::def_id::DefId,
    tcx: rustc_middle::ty::TyCtxt<'tcx>,
) -> bool {
    let is_linked = memory.has_linked_value(place_linked);
    if !is_linked {
        debug!("PLACE {place_linked:?} IS NOT LINKED SO PLACE {place_to_link:?} DOES NOT HAVE THE SAME VALUE");
        return false;
    }
    let has_same_type =
        check_aggregate_of_same_type(place_to_link, place_linked, caller_function_def_id, tcx);
    if !has_same_type {
        debug!("THE TYPES OF PLACE {place_to_link:?} AND {place_linked:?} DO NOT MATCH SO AS TO HAVE THE SAME VALUE");
        return false;
    }
    true // has same type and the place is linked
}

/// Handles MIR assignments of the form: `_X = { copy_data: move _Y }`.
/// Create a new aggregate value (tuple, array, `std::sync::Arc`, etc.) from the places in the operands.
///
/// The function maps the places in the operands to a vector that contains:
/// - Every place with a sync variable
/// - A `None` for non-sync variables values
/// - It respects the order in which the places appear (i.e. the order of the fields in the aggregate)
///
/// It passes it on to the function memory to create the corresponding aggregate
pub fn handle_aggregate_assignment<'tcx>(
    place: &rustc_middle::mir::Place<'tcx>,
    operands: &Vec<rustc_middle::mir::Operand<'tcx>>,
    memory: &mut Memory,
) {
    let mut has_linked_value = false;
    let mut aggregate_fields: Vec<Option<rustc_middle::mir::Place<'tcx>>> = Vec::new();

    for operand in operands {
        // Extract the place to be assigned
        let rhs = match operand {
            rustc_middle::mir::Operand::Copy(place) | rustc_middle::mir::Operand::Move(place) => {
                place
            }
            // Nothing to do if we found a constant as one of the operands.
            rustc_middle::mir::Operand::Constant(_) => continue,
        };
        if memory.has_linked_value(rhs) {
            has_linked_value = true;
            aggregate_fields.push(Some(*rhs));
        } else {
            aggregate_fields.push(None);
        }
    }

    if has_linked_value {
        memory.create_aggregate(*place, &aggregate_fields);
    }
}

/// Checks if `place_to_link` contains a mutex, a mutex guard, a join handle or a condition variable.
/// If `place_to_link` contains a synchronization variable, links it to `place_linked`.
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
    place_to_link: &rustc_middle::mir::Place<'tcx>,
    place_linked: &rustc_middle::mir::Place<'tcx>,
    memory: &mut Memory,
    caller_function_def_id: rustc_hir::def_id::DefId,
    tcx: rustc_middle::ty::TyCtxt<'tcx>,
) {
    if should_link_to_same_value(
        place_to_link,
        place_linked,
        memory,
        caller_function_def_id,
        tcx,
    ) {
        memory.link_place_to_same_value(*place_to_link, *place_linked);
    }
}

/// Checks if the first argument for a function call contains a mutex, a mutex guard,
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
    args: &[rustc_span::source_map::Spanned<rustc_middle::mir::Operand<'tcx>>],
    return_value: rustc_middle::mir::Place<'tcx>,
    memory: &mut Memory,
    caller_function_def_id: rustc_hir::def_id::DefId,
    tcx: rustc_middle::ty::TyCtxt<'tcx>,
) {
    let Some(first_argument) = extract_nth_argument_as_place(args, 0) else {
        // Nothing to check: Either the first argument is not present or it is a constant.
        return;
    };

    if should_link_to_same_value(
        &return_value,
        &first_argument,
        memory,
        caller_function_def_id,
        tcx,
    ) {
        memory.link_place_to_same_value(return_value, first_argument);
    }
}
