//! Submodule for functions that require special treatment
//! during the translation.
//!
//! These could be synchronization primitives, `panic!`-related primitives,
//! or simply functions for which we are not interested in translation the source code.
//! For example: Calls to standard library methods, iterators, etc.

use crate::translator::error_handling::handle_err_add_arc;
use crate::translator::naming::{
    function_diverging_call_transition_label, function_panic_transition_label,
};
use netcrab::petri_net::{PetriNet, PlaceRef, TransitionRef};

const FUNCTIONS_EXCLUDED_FROM_TRANSLATION: [&str; 1] = ["std::iter::ExactSizeIterator::len"];

const PANIC_FUNCTIONS: [&str; 5] = [
    "core::panicking::assert_failed",
    "core::panicking::panic",
    "core::panicking::panic_fmt",
    "std::rt::begin_panic",
    "std::rt::begin_panic_fmt",
];

/// Checks whether the function name corresponds to one of the functions
/// that needs to be translated separately, e.g, mutex functions.
fn is_function_excluded_from_translation(function_name: &str) -> bool {
    for name in FUNCTIONS_EXCLUDED_FROM_TRANSLATION {
        if function_name == name {
            return true;
        }
    }
    false
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
    tcx: rustc_middle::ty::TyCtxt,
) -> bool {
    let function_name = tcx.def_path_str(function_def_id);
    tcx.is_foreign_item(function_def_id)
        || !tcx.is_mir_available(function_def_id)
        || is_function_excluded_from_translation(&function_name)
}

/// Creates an abridged Petri net representation of a function call.
/// Connects the start place and end place through a new transition.
/// If an optional cleanup place is provided, it connects the transition to this place too.
/// Returns the transition reference representing the function call.
pub fn call_foreign_function(
    start_place: &PlaceRef,
    end_place: &PlaceRef,
    cleanup_place: Option<PlaceRef>,
    transition_label: &str,
    net: &mut PetriNet,
) -> TransitionRef {
    let transition_foreign_call = net.add_transition(transition_label);
    net.add_arc_place_transition(start_place, &transition_foreign_call)
        .unwrap_or_else(|_| {
            handle_err_add_arc("foreign call start place", "foreign call transition");
        });
    net.add_arc_transition_place(&transition_foreign_call, end_place)
        .unwrap_or_else(|_| {
            handle_err_add_arc("foreign call transition", "foreign call end place");
        });

    if let Some(cleanup_place) = cleanup_place {
        net.add_arc_transition_place(&transition_foreign_call, &cleanup_place)
            .unwrap_or_else(|_| {
                handle_err_add_arc("foreign call transition", "cleanup place");
            });
    }

    transition_foreign_call
}

/// Creates an abridged Petri net representation of a diverging function call.
/// Connects the start place to a new transition that models a call to a function which does not return.
pub fn call_diverging_function(start_place: &PlaceRef, function_name: &str, net: &mut PetriNet) {
    let label = &function_diverging_call_transition_label(function_name);
    let transition = net.add_transition(label);
    net.add_arc_place_transition(start_place, &transition)
        .unwrap_or_else(|_| {
            handle_err_add_arc("diverging call start place", "diverging call transition");
        });
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
    let transition = net.add_transition(&function_panic_transition_label(function_name));
    net.add_arc_place_transition(start_place, &transition)
        .unwrap_or_else(|_| handle_err_add_arc("panic start place", "panic transition"));
    net.add_arc_transition_place(&transition, unwind_place)
        .unwrap_or_else(|_| handle_err_add_arc("panic transition", "panic place"));
}
