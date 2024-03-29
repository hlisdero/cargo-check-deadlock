//! Submodule for functions that require special treatment
//! during the translation.
//!
//! These could be `panic!`-related primitives, diverging functions
//! or simply functions which we are not interested in translating.
//! For example: Calls to standard library methods, iterators, etc.

use crate::data_structures::petri_net_interface::{
    add_arc_place_transition, connect_places, PetriNet, PlaceRef,
};
use crate::naming::function::{
    diverging_call_transition_label, foreign_call_transition_labels, panic_transition_label,
};
use crate::translator::function::{Places, Transitions};

/// Checks whether the function name corresponds to one of the functions
/// that starts a panic, i.e. an unwind of the stack.
pub fn is_panic_function(function_name: &str) -> bool {
    matches!(
        function_name,
        "core::panicking::assert_failed"
            | "core::panicking::panic"
            | "core::panicking::panic_fmt"
            | "std::rt::begin_panic"
            | "std::rt::begin_panic_fmt"
    )
}

/// Checks whether the function with the given `DefId` should be treated
/// as a foreign function call.
///
/// A foreign function call occurs when:
/// - the function belongs to the standard library.
/// - the function belongs to the core library.
/// - the function belongs to the `alloc` crate, the core allocation and collections library.
/// - the function does not have a MIR representation.
/// - the function is a foreign item i.e., linked via extern { ... }).
pub fn is_foreign_function(
    function_def_id: rustc_hir::def_id::DefId,
    function_name: &str,
    tcx: rustc_middle::ty::TyCtxt,
) -> bool {
    function_name.starts_with("std::")
        || function_name.starts_with("core::")
        || function_name.starts_with("alloc::")
        || tcx.is_foreign_item(function_def_id)
        || !tcx.is_mir_available(function_def_id)
}

/// Creates an abridged Petri net representation of a function call.
/// Connects the start place and end place through a new transition.
/// If an optional cleanup place is provided, it connects the start
/// place and cleanup place through a second new transition.
/// The labels from the transition are generated from the function name and the index.
///
/// Returns the transition representing the function call.
pub fn call_foreign_function(
    function_name: &str,
    index: usize,
    places: Places,
    net: &mut PetriNet,
) -> Transitions {
    let (default_label, cleanup_label) = foreign_call_transition_labels(function_name, index);
    match places {
        Places::Basic {
            start_place,
            end_place,
        } => {
            let default = connect_places(net, &start_place, &end_place, &default_label);
            Transitions::Basic { default }
        }
        Places::WithCleanup {
            start_place,
            end_place,
            cleanup_place,
        } => {
            let default = connect_places(net, &start_place, &end_place, &default_label);
            let cleanup = connect_places(net, &start_place, &cleanup_place, &cleanup_label);
            Transitions::WithCleanup { default, cleanup }
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
