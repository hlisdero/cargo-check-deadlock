//! Submodule that defines the naming of places and transitions in the Petri net
//! that concern the statements inside the basic blocks of a MIR function.
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

/// Label of the transition of any `Statement`.
#[inline]
pub fn transition_label(function_name: &str, block_index: usize, statement_index: usize) -> String {
    format!(
        "{}_BB{block_index}_STMT{statement_index}",
        sanitize(function_name)
    )
}

/// Label of the end place of any `Statement`.
#[inline]
pub fn end_place_label(function_name: &str, block_index: usize, statement_index: usize) -> String {
    format!(
        "{}_BB{block_index}_STMT{statement_index}_END",
        sanitize(function_name)
    )
}
