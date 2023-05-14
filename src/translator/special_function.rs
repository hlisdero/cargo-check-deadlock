//! Submodule for functions that require special treatment
//! during the translation.
//!
//! These could be `panic!`-related primitives, diverging functions
//! or simply functions which we are not interested in translating.
//! For example: Calls to standard library methods, iterators, etc.

use crate::data_structures::petri_net_interface::{add_arc_place_transition, connect_places};
use crate::data_structures::petri_net_interface::{PetriNet, PlaceRef};
use crate::naming::function::{diverging_call_transition_label, panic_transition_label};
use crate::translator::function::{Places, Transitions};

const PANIC_FUNCTIONS: [&str; 5] = [
    "core::panicking::assert_failed",
    "core::panicking::panic",
    "core::panicking::panic_fmt",
    "std::rt::begin_panic",
    "std::rt::begin_panic_fmt",
];

/// Checks whether the function name corresponds to one of the functions
/// that should be excluded from the translation.
///
/// These are:
/// - All the standard library functions.
/// - All the core library functions.
/// - All the functions in `alloc`, the core allocation and collections library.
#[inline]
fn is_function_excluded_from_translation(function_name: &str) -> bool {
    function_name.starts_with("std::")
        || function_name.starts_with("core::")
        || function_name.starts_with("alloc::")
}

/// Checks whether the function name corresponds to one of the functions
/// that starts a panic, i.e. an unwind of the stack.
pub fn is_panic_function(function_name: &str) -> bool {
    for name in PANIC_FUNCTIONS {
        if function_name == name {
            return true;
        }
    }
    false
}

/// Checks whether the function with the given `DefId` should be treated
/// as a foreign function call.
///
/// A foreign function call occurs when:
/// - the function does not have a MIR representation.
/// - the function is a foreign item i.e., linked via extern { ... }).
/// - the function belongs to the exclusions listed in `FUNCTIONS_EXCLUDED_FROM_TRANSLATION`
pub fn is_foreign_function(
    function_def_id: rustc_hir::def_id::DefId,
    function_name: &str,
    tcx: rustc_middle::ty::TyCtxt,
) -> bool {
    is_function_excluded_from_translation(function_name)
        || tcx.is_foreign_item(function_def_id)
        || !tcx.is_mir_available(function_def_id)
}

/// Creates an abridged Petri net representation of a function call.
/// Connects the start place and end place through a new transition.
/// If an optional cleanup place is provided, it connects the start
/// place and cleanup place through a second new transition.
///
/// Returns the transition representing the function call.
pub fn call_foreign_function(
    places: Places,
    transition_labels: &(String, String),
    net: &mut PetriNet,
) -> Transitions {
    match places {
        Places::Basic {
            start_place,
            end_place,
        } => {
            let transition = connect_places(net, &start_place, &end_place, &transition_labels.0);
            Transitions::Basic { transition }
        }
        Places::WithCleanup {
            start_place,
            end_place,
            cleanup_place,
        } => {
            let transition = connect_places(net, &start_place, &end_place, &transition_labels.0);
            let cleanup_transition =
                connect_places(net, &start_place, &cleanup_place, &transition_labels.1);

            Transitions::WithCleanup {
                transition,
                cleanup_transition,
            }
        }
    }
}

/// Creates an abridged Petri net representation of a diverging function call.
/// Connects the start place to a new transition that models a call to a function which does not return.
pub fn call_diverging_function(start_place: &PlaceRef, function_name: &str, net: &mut PetriNet) {
    let label = &diverging_call_transition_label(function_name);
    let transition = net.add_transition(label);
    add_arc_place_transition(net, start_place, &transition);
}

/// Creates an abridged Petri net representation of a function call
/// that starts a panic, i.e. an unwind of the stack.
/// Connects the start place to the panic place through a new transition.
pub fn call_panic_function(
    start_place: &PlaceRef,
    unwind_place: &PlaceRef,
    function_name: &str,
    net: &mut PetriNet,
) {
    let label = panic_transition_label(function_name);
    connect_places(net, start_place, unwind_place, &label);
}
