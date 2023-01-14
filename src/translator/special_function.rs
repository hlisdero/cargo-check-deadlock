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

const SUPPORTED_SPECIAL_FUNCTIONS: [&str; 1] = ["std::iter::ExactSizeIterator::len"];

const PANIC_FUNCTIONS: [&str; 5] = [
    "core::panicking::assert_failed",
    "core::panicking::panic",
    "core::panicking::panic_fmt",
    "std::rt::begin_panic",
    "std::rt::begin_panic_fmt",
];

/// Checks whether the function name corresponds to one of the functions
/// that needs to be translated separately, e.g, mutex functions.
pub fn is_special(function_name: &str) -> bool {
    for name in SUPPORTED_SPECIAL_FUNCTIONS {
        if function_name == name {
            return true;
        }
    }
    false
}

/// Checks whether the function name corresponds to one of the functions
/// that starts a panic, i.e. an unwind of the stack.
pub fn is_panic(function_name: &str) -> bool {
    for name in PANIC_FUNCTIONS {
        if function_name == name {
            return true;
        }
    }
    false
}

/// Creates an abridged Petri net representation of a function call.
/// Connects the start place and end place through a new transition.
/// If an optional cleanup place is provided, it connects the transition to this place too.
/// Returns the transition reference representing the function call.
pub fn foreign_function_call(
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
pub fn diverging_function_call(start_place: &PlaceRef, function_name: &str, net: &mut PetriNet) {
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
pub fn panic_function_call(
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
