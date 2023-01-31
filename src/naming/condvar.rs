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

/// Labels of the four places that model every `Condvar`.
/// `P1` -> `lost_signal_possible`.
/// `P2` -> `signal_input`.
/// `P3` -> `wait_input`.
/// `P4` -> `signal_output`
#[inline]
pub fn place_labels(index: usize) -> (String, String, String, String) {
    (
        format!("CONDVAR_{index}_P1"),
        format!("CONDVAR_{index}_P2"),
        format!("CONDVAR_{index}_P3"),
        format!("CONDVAR_{index}_P4"),
    )
}

/// Labels of the two transitions that model every `Condvar`.
/// `T1` -> `lost_signal_transition`.
/// `T2` -> `signal_transition`.
#[inline]
pub fn transition_labels(index: usize) -> (String, String) {
    (format!("CONDVAR_{index}_T1"), format!("CONDVAR_{index}_T2"))
}

/// Label of the transitions that represent a call to `std::sync::Condvar::new`.
#[inline]
pub fn new_transition_labels(index: usize) -> (String, String) {
    (
        format!("std_sync_Condvar_new_{index}"),
        format!("std_sync_Condvar_new_{index}_UNWIND"),
    )
}

/// Label of the transitions that represent a call to `std::sync::Condvar::notify_one`.
#[inline]
pub fn notify_one_transition_label(index: usize) -> String {
    format!("std_sync_Condvar_notify_one_{index}")
}

/// Label of the transitions that represent a call to `std::sync::Condvar::wait`.
#[inline]
pub fn wait_transition_labels(index: usize) -> (String, String) {
    (
        format!("std_sync_Condvar_wait_{index}_START"),
        format!("std_sync_Condvar_wait_{index}_END"),
    )
}
