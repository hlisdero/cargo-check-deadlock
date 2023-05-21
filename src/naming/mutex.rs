//! Submodule that defines the naming of places and transitions in the Petri net
//! that concern the translation of functions related to mutexes.
//!
//! These functions are called every time that a new place or transition
//! in the resulting net is created.
//! This ensures a consistent naming and provides a centralized place to tweak
//! the configuration if needed.
//!
//! All functions listed here should have an `#[inline]` attribute for performance reasons.
//! See the reference for more information:
//! <https://doc.rust-lang.org/stable/reference/attributes/codegen.html>

/// Label of the single place that models every `Mutex`.
#[inline]
pub fn place_label(index: usize) -> String {
    format!("MUTEX_{index}")
}

/// Labels of the two places that model the condition (the value)
/// stored inside a `Mutex` used in conjunction with a condition variable.
#[inline]
pub fn condition_place_labels(index: usize) -> (String, String) {
    (
        format!("MUTEX_{index}_CONDITION_NOT_SET"),
        format!("MUTEX_{index}_CONDITION_SET"),
    )
}
