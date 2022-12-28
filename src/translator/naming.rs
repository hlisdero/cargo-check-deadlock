//! Submodule that defines the naming of places in the Petri net.
//!
//! These functions are called every time that a new place
//! in the resulting net is created.
//! This ensures a consistent naming and provides a centralized place to tweak
//! the configuration if needed.
//!
//! All functions listed here should have an `#[inline]` attribute for performance reasons.
//! See the reference for more information:
//! <https://doc.rust-lang.org/stable/reference/attributes/codegen.html>

/// Name of the place that models the program start state.
/// It is present in every generated Petri net.
pub const PROGRAM_START: &str = "PROGRAM_START";
/// Name of the place that models the normal program end state.
/// It is present in every generated Petri net.
pub const PROGRAM_END: &str = "PROGRAM_END";
/// Name of the place that models the program end state after a `panic!`.
/// It is present in every generated Petri net.
pub const PROGRAM_PANIC: &str = "PROGRAM_PANIC";

/// Name of the transition for an empty `BasicBlock` with no statements.
pub const BASIC_BLOCK_EMPTY: &str = "BASIC_BLOCK_EMPTY";
/// Name of the end place of any `BasicBlock`.
pub const BASIC_BLOCK_END_PLACE: &str = "BASIC_BLOCK_END_PLACE";

/// Name of the end place of any `Statement`.
pub const STATEMENT_END: &str = "STATEMENT_END";

#[inline]
pub fn mutex_place_label_from_index(index: usize) -> String {
    format!("MUTEX_{index}")
}

#[inline]
pub fn function_transition_label_from_function_name(function_name: &str) -> String {
    format!("RUN_{function_name}")
}

#[inline]
pub fn statement_transition_label_from_statement_kind(
    statement_kind: &rustc_middle::mir::StatementKind,
) -> String {
    format!("STATEMENT {:?}", statement_kind)
}
