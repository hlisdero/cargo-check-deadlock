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

/// Label of the transitions that represent a call to `std::sync::Mutex::<T>::new`.
#[inline]
pub fn new_transition_labels(index: usize) -> (String, String) {
    (
        format!("std_sync_Mutex_T_new_{index}"),
        format!("std_sync_Mutex_T_new_{index}_UNWIND"),
    )
}

/// Label of the transitions that represent a call to `std::sync::Mutex::<T>::lock`.
#[inline]
pub fn lock_transition_labels(index: usize) -> (String, String) {
    (
        format!("std_sync_Mutex_T_lock_{index}"),
        format!("std_sync_Mutex_T_lock_{index}_UNWIND"),
    )
}
