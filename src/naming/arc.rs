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

/// Label of the transition that represents a call to `std::sync::Arc::<T>::new`.
#[inline]
pub fn new_transition_label(index: usize) -> String {
    format!("std_sync_Arc_T_new_{index}")
}

/// Label of the unwind transition for a call to `std::sync::Arc::<T>::new`.
#[inline]
pub fn new_unwind_transition_label(index: usize) -> String {
    format!("std_sync_Arc_T_new_{index}_UNWIND")
}

/// Label of the transition that represents a call to `std::clone::Clone::clone`.
#[inline]
pub fn clone_transition_label(index: usize) -> String {
    format!("std_clone_Clone_clone_{index}")
}

/// Label of the unwind transition for a call to `std::clone::Clone::clone`.
#[inline]
pub fn clone_unwind_transition_label(index: usize) -> String {
    format!("std_clone_Clone_clone_{index}_UNWIND")
}

/// Label of the transition that represents a call to `std::ops::Deref::deref`.
#[inline]
pub fn deref_transition_label(index: usize) -> String {
    format!("std_ops_Deref_deref_{index}")
}

/// Label of the unwind transition for a call to `std::ops::Deref::deref`.
#[inline]
pub fn deref_unwind_transition_label(index: usize) -> String {
    format!("std_ops_Deref_deref_{index}_UNWIND")
}
