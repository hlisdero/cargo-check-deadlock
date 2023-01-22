//! Submodule that defines the naming of places and transitions in the Petri net
//! that concern the translation of functions related to `std::sync::Arc`.
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

/// Label of the transition that represents a call to a function of `std::sync::Arc`.
#[inline]
pub fn function_transition_label(function_name: &str, index: usize) -> String {
    format!("{}_{index}", sanitize(function_name))
}