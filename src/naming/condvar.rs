//! Submodule that defines the naming of places and transitions in the Petri net
//! that concern the translation of functions related to condition variables.
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

/// Labels of the four places that model every `Condvar`.
#[inline]
pub fn place_labels(index: usize) -> (String, String, String, String) {
    (
        format!("CONDVAR_{index}_WAIT_ENABLED"),
        format!("CONDVAR_{index}_NOTIFY"),
        format!("CONDVAR_{index}_WAITING"),
        format!("CONDVAR_{index}_WAITING_FOR_LOCK"),
    )
}

/// Labels of the two transitions that model every `Condvar`.
#[inline]
pub fn transition_labels(index: usize) -> (String, String) {
    (
        format!("CONDVAR_{index}_LOST_SIGNAL"),
        format!("CONDVAR_{index}_NOTIFY_RECEIVED"),
    )
}

/// Label of the transitions that represent a call to `std::sync::Condvar::wait` or `std::sync::Condvar::wait_while`.
#[inline]
pub fn wait_transition_labels(function_name: &str, index: usize) -> (String, String, String) {
    (
        format!("{}_{index}_START", sanitize(function_name)),
        format!("{}_{index}_END", sanitize(function_name)),
        format!("{}_{index}_SKIP", sanitize(function_name)),
    )
}
