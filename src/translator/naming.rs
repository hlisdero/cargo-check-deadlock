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

/// Names of the basic places present in every generated Petri net.
pub const PROGRAM_START: &str = "PROGRAM_START";
pub const PROGRAM_END: &str = "PROGRAM_END";
pub const PROGRAM_PANIC: &str = "PROGRAM_PANIC";

#[inline]
pub fn mutex_place_label_from_index(index: usize) -> String {
    format!("MUTEX_{index}")
}

#[inline]
pub fn function_transition_label_from_function_name(function_name: &str) -> String {
    format!("RUN_{function_name}")
}
