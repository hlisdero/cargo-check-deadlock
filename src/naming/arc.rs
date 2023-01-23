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

/// Label of the transitions that represent a call to `std::sync::Arc::<T>::new`.
#[inline]
pub fn new_transition_labels(index: usize) -> (String, String) {
    (
        format!("std_sync_Arc_T_new_{index}"),
        format!("std_sync_Arc_T_new_{index}_UNWIND"),
    )
}

/// Label of the transitions that represent a call to `std::clone::Clone::clone`.
#[inline]
pub fn clone_transition_labels(index: usize) -> (String, String) {
    (
        format!("std_clone_Clone_clone_{index}"),
        format!("std_clone_Clone_clone_{index}_UNWIND"),
    )
}

/// Label of the transitions that represent a call to `std::ops::Deref::deref`.
#[inline]
pub fn deref_transition_labels(index: usize) -> (String, String) {
    (
        format!("std_ops_Deref_deref_{index}"),
        format!("std_ops_Deref_deref_{index}_UNWIND"),
    )
}
