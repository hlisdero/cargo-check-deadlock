//! Submodule that defines the naming of places and transitions in the Petri net
//! that concern function calls (with or without a MIR representation).
//!
//! These functions are called every time that a new place or transition
//! in the resulting net is created.
//! This ensures a consistent naming and provides a centralized place to tweak
//! the configuration if needed.
//!
//! All functions listed here should have an `#[inline]` attribute for performance reasons.
//! See the reference for more information:
//! <https://doc.rust-lang.org/stable/reference/attributes/codegen.html>

use super::sanitize;

/// Indexed function name for a call to a MIR function.
#[inline]
pub fn indexed_mir_function_name(function_name: &str, index: usize) -> String {
    format!("{}_{index}", sanitize(function_name))
}

#[inline]
pub fn indexed_mir_function_cleanup_label(function_name: &str, index: usize) -> String {
    format!("{}_{index}_CALL_UNWIND", sanitize(function_name))
}

/// Label of the transition for the return statement of a function.
#[inline]
pub fn return_transition_label(function_name: &str) -> String {
    format!("{}_RETURN", sanitize(function_name))
}

/// Label of the transitions for a call to a foreign function.
#[inline]
pub fn foreign_call_transition_labels(function_name: &str, index: usize) -> (String, String) {
    (
        format!("{}_{index}_CALL", sanitize(function_name)),
        format!("{}_{index}_CALL_UNWIND", sanitize(function_name)),
    )
}

/// Label of the transition that represents a diverging function call (a function that does not return).
#[inline]
pub fn diverging_call_transition_label(function_name: &str) -> String {
    format!("{}_DIVERGING_CALL", sanitize(function_name))
}

/// Label of the transition that represents a call to a `panic!`.
#[inline]
pub fn panic_transition_label(function_name: &str) -> String {
    format!("{}_PANIC", sanitize(function_name))
}
