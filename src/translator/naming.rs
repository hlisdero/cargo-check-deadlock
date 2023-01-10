//! Submodule that defines the naming of places and transitions in the Petri net.
//!
//! These functions are called every time that a new place
//! in the resulting net is created.
//! This ensures a consistent naming and provides a centralized place to tweak
//! the configuration if needed.
//!
//! All functions listed here should have an `#[inline]` attribute for performance reasons.
//! See the reference for more information:
//! <https://doc.rust-lang.org/stable/reference/attributes/codegen.html>

/// Label of the place that models the program start state.
/// It is present in every generated Petri net.
pub const PROGRAM_START: &str = "PROGRAM_START";
/// Label of the place that models the normal program end state.
/// It is present in every generated Petri net.
pub const PROGRAM_END: &str = "PROGRAM_END";
/// Label of the place that models the program end state after a `panic!`.
/// It is present in every generated Petri net.
pub const PROGRAM_PANIC: &str = "PROGRAM_PANIC";

/// Label of the transition for the return statement of a function.
#[inline]
pub fn function_return_transition_label(function_name: &str) -> String {
    format!("{}_RETURN", sanitize(function_name))
}

/// Label of the transition for a foreign function item.
#[inline]
pub fn function_foreign_call_transition_label(function_name: &str) -> String {
    format!("{}_FOREIGN_CALL", sanitize(function_name))
}

/// Label of the start place of any `BasicBlock`.
#[inline]
pub fn basic_block_start_place_label(function_name: &str, index: usize) -> String {
    format!("{}_BASIC_BLOCK_{index}", sanitize(function_name))
}

/// Label of the end place of any `BasicBlock`.
#[inline]
pub fn basic_block_end_place_label(function_name: &str, index: usize) -> String {
    format!("{}_BASIC_BLOCK_END_PLACE_{index}", sanitize(function_name))
}

/// Label of the transition that represents a goto terminator to another `BasicBlock`.
#[inline]
pub fn basic_block_goto_transition_label(function_name: &str, index: usize) -> String {
    format!("{}_GOTO_{index}", sanitize(function_name))
}

/// Label of the transition that represents a switch int terminator to another `BasicBlock`.
#[inline]
pub fn basic_block_switch_int_transition_label(function_name: &str, index: usize) -> String {
    format!("{}_SWITCH_INT_{index}", sanitize(function_name))
}

/// Label of the transition that represents an unwind terminator to the general `PROGRAM_PANIC` place.
#[inline]
pub fn basic_block_unwind_transition_label(function_name: &str, index: usize) -> String {
    format!("{}_UNWIND_{index}", sanitize(function_name))
}

/// Label of the transition that represents a drop terminator.
#[inline]
pub fn basic_block_drop_transition_label(function_name: &str, index: usize) -> String {
    format!("{}_DROP_{index}", sanitize(function_name))
}

/// Label of the transition that represents the (optional) unwind path of a drop terminator.
#[inline]
pub fn basic_block_drop_unwind_transition_label(function_name: &str, index: usize) -> String {
    format!("{}_DROP_UNWIND_{index}", sanitize(function_name))
}

/// Label of the transition that represents an assert terminator.
#[inline]
pub fn basic_block_assert_transition_label(function_name: &str, index: usize) -> String {
    format!("{}_ASSERT_{index}", sanitize(function_name))
}

/// Label of the transition that represents the (optional) unwind path of an assert terminator.
#[inline]
pub fn basic_block_assert_cleanup_transition_label(function_name: &str, index: usize) -> String {
    format!("{}_ASSERT_CLEANUP_{index}", sanitize(function_name))
}

/// Label of the transition that represents a diverging function call (a function that does not return).
#[inline]
pub fn basic_block_diverging_call_transition_label(function_name: &str) -> String {
    format!("{}_DIVERGING_CALL", sanitize(function_name))
}

/// Label of the transition of any `Statement`.
#[inline]
pub fn statement_transition_label(
    function_name: &str,
    block_index: usize,
    statement_index: usize,
) -> String {
    format!(
        "{}_BLOCK_{block_index}_STATEMENT_{statement_index}",
        sanitize(function_name)
    )
}

/// Label of the end place of any `Statement`.
#[inline]
pub fn statement_end_place_label(
    function_name: &str,
    block_index: usize,
    statement_index: usize,
) -> String {
    format!(
        "{}_BLOCK_{block_index}_STATEMENT_END_{statement_index}",
        sanitize(function_name)
    )
}

/// Label of the single place that models every `Mutex`.
#[inline]
pub fn mutex_place_label(index: usize) -> String {
    format!("MUTEX_{index}")
}

/// Label of the transition that represents a call to a function of `std::sync::Mutex<T>`.
#[inline]
pub fn mutex_function_transition_label(function_name: &str, index: usize) -> String {
    format!("{}_{index}", sanitize(function_name))
}

/// Sanitize the function name for the DOT format:
/// - Replace colons with underscores.
/// - Replace generic types "<T>" with "T" .
fn sanitize(function_name: &str) -> String {
    function_name.replace("::", "_").replace("<T>", "T")
}
