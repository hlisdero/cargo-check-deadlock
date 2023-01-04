//! Constant definitions for functions that require special treatment
//! during the translation.
//!
//! These could be synchronization primitives or `panic!`-related primitives.
use crate::translator::error_handling::handle_err_add_arc;
use crate::translator::naming::function_foreign_call_transition_label;
use netcrab::petri_net::{PetriNet, PlaceRef};

const SUPPORTED_SPECIAL_FUNCTIONS: [&str; 3] = [
    "std::sync::Mutex::<T>::new",
    "std::sync::Mutex::<T>::lock",
    "std::sync::Mutex::<T>::try_lock",
];

const PANIC_FUNCTIONS: [&str; 5] = [
    "core::panicking::assert_failed",
    "core::panicking::panic",
    "core::panicking::panic_fmt",
    "std::rt::begin_panic",
    "std::rt::begin_panic_fmt",
];

/// Check whether the function name corresponds to one of the functions
/// that needs to be translated separately, e.g, mutex functions.
pub fn is_special(function_name: &str) -> bool {
    for name in SUPPORTED_SPECIAL_FUNCTIONS {
        if function_name == name {
            return true;
        }
    }
    false
}

/// Check whether the function name corresponds to one of the functions
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
pub fn foreign_function_call(
    function_name: &str,
    start_place: &PlaceRef,
    end_place: &PlaceRef,
    cleanup_place: Option<PlaceRef>,
    net: &mut PetriNet,
) {
    let transition_foreign_call =
        net.add_transition(&function_foreign_call_transition_label(function_name));
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
}
