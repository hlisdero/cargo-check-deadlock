//! Submodule for implementing the translation of synchronization primitives
//! and the translation of thread primitives.

pub mod condvar;
pub mod mutex;
pub mod thread;

use log::debug;

use crate::data_structures::petri_net_interface::PetriNet;
use crate::translator::function::{Places, PostprocessingTask};
use crate::translator::mir_function::memory::Memory;
use crate::utils::{
    check_substring_in_place_type, extract_nth_argument_as_place, get_field_number_in_projection,
};

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
            | "std::thread::spawn"
            | "std::thread::JoinHandle::<T>::join"
    )
}

/// Calls the corresponding handler for the supported synchronization or multithreading functions.
pub fn call_function<'tcx>(
    function_name: &str,
    index: usize,
    args: &[rustc_middle::mir::Operand<'tcx>],
    destination: rustc_middle::mir::Place<'tcx>,
    places: Places,
    net: &mut PetriNet,
    memory: &mut Memory<'tcx>,
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

/// Checks whether a place contains a sync variable
/// (mutex, mutex guard, join handle or condition variable)
pub fn check_if_sync_variable<'tcx>(
    place: &rustc_middle::mir::Place<'tcx>,
    caller_function_def_id: rustc_hir::def_id::DefId,
    tcx: rustc_middle::ty::TyCtxt<'tcx>,
) -> bool {
    check_substring_in_place_type(place, "std::sync::MutexGuard<", caller_function_def_id, tcx)
        || check_substring_in_place_type(place, "std::sync::Mutex<", caller_function_def_id, tcx)
        || check_substring_in_place_type(
            place,
            "std::thread::JoinHandle<",
            caller_function_def_id,
            tcx,
        )
        || check_substring_in_place_type(place, "std::sync::Condvar", caller_function_def_id, tcx)
}

/// Handles MIR assignments of the form: `_X = { copy_data: move _Y }`.
/// Create a new aggregate value (tuple, array, `std::sync::Arc`, etc.) from the sync variables in the operands.
/// If the operand in the right hand side contains a sync variable, the function includes it in the aggregate.
pub fn handle_aggregate_assignment<'tcx>(
    place: &rustc_middle::mir::Place<'tcx>,
    operands: &Vec<rustc_middle::mir::Operand<'tcx>>,
    memory: &mut Memory<'tcx>,
    caller_function_def_id: rustc_hir::def_id::DefId,
    tcx: rustc_middle::ty::TyCtxt<'tcx>,
) {
    let mut places_with_sync_variables: Vec<rustc_middle::mir::Place<'tcx>> = Vec::new();

    for operand in operands {
        // Extract the place to be assigned
        let rhs = match operand {
            rustc_middle::mir::Operand::Copy(place) | rustc_middle::mir::Operand::Move(place) => {
                place
            }
            // Nothing to do if we found a constant as one of the operands.
            rustc_middle::mir::Operand::Constant(_) => continue,
        };
        if check_if_sync_variable(rhs, caller_function_def_id, tcx) {
            places_with_sync_variables.push(*rhs);
        }
    }

    if !places_with_sync_variables.is_empty() {
        memory.create_aggregate(*place, &places_with_sync_variables);
        debug!("CREATED AGGREGATE AT {place:?} WITH PLACES {places_with_sync_variables:?}");
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
    memory: &mut Memory<'tcx>,
    caller_function_def_id: rustc_hir::def_id::DefId,
    tcx: rustc_middle::ty::TyCtxt<'tcx>,
) {
    if !check_if_sync_variable(place_to_link, caller_function_def_id, tcx) {
        return;
    }
    if place_linked.has_deref() {
        // Checks if the place has a `ProjectionElem::Deref`
        let field_number = get_field_number_in_projection(place_linked);
        // Create a new place without the projections
        let mut base_place = *place_linked;
        base_place.projection = rustc_middle::ty::List::empty();

        debug!("ACCESS FIELD {field_number} AFTER DEREF IN BASE PLACE {base_place:?}");
        memory.link_field_in_aggregate(*place_to_link, base_place, field_number);
    } else {
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
