//! Submodule that defines the naming of places and transitions in the Petri net
//! that concern the basic blocks inside a MIR function.
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

/// Label of the start place of any `BasicBlock`.
#[inline]
pub fn start_place_label(function_name: &str, index: usize) -> String {
    format!("{}_BB{index}", sanitize(function_name))
}

/// Label of the end place of any `BasicBlock`.
#[inline]
pub fn end_place_label(function_name: &str, index: usize) -> String {
    format!("{}_BB{index}_END_PLACE", sanitize(function_name))
}

/// Label of the transition that represents a goto terminator to another `BasicBlock`.
#[inline]
pub fn goto_transition_label(function_name: &str, index: usize) -> String {
    format!("{}_GOTO_{index}", sanitize(function_name))
}

/// Label of the transition that represents a switch int terminator to another `BasicBlock`.
#[inline]
pub fn switch_int_transition_label(function_name: &str, index: usize) -> String {
    format!("{}_SWITCH_INT_{index}", sanitize(function_name))
}

/// Label of the transition that represents an unwind terminator to the general `PROGRAM_PANIC` place.
#[inline]
pub fn unwind_transition_label(function_name: &str, index: usize) -> String {
    format!("{}_UNWIND_{index}", sanitize(function_name))
}

/// Label of the transition that represents a drop terminator.
#[inline]
pub fn drop_transition_label(function_name: &str, index: usize) -> String {
    format!("{}_DROP_{index}", sanitize(function_name))
}

/// Label of the transition that represents the (optional) unwind path of a drop terminator.
#[inline]
pub fn drop_cleanup_transition_label(function_name: &str, index: usize) -> String {
    format!("{}_DROP_UNWIND_{index}", sanitize(function_name))
}

/// Label of the transition that represents an assert terminator.
#[inline]
pub fn assert_transition_label(function_name: &str, index: usize) -> String {
    format!("{}_ASSERT_{index}", sanitize(function_name))
}

/// Label of the transition that represents the (optional) unwind path of an assert terminator.
#[inline]
pub fn assert_cleanup_transition_label(function_name: &str, index: usize) -> String {
    format!("{}_ASSERT_CLEANUP_{index}", sanitize(function_name))
}

/// Label of the transition that represents the `Unreachable` terminator.
#[inline]
pub fn unreachable_transition_label(function_name: &str, index: usize) -> String {
    format!("{}_UNREACHABLE_{index}", sanitize(function_name))
}
