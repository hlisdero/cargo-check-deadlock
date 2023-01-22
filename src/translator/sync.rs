//! Submodule for implementing the translation of synchronization primitives.
//!
//! It defines which functions are supported and how to detect them.
//! It exposes only the necessary definitions to the outside modules.

mod arc_manager;
mod mutex;
mod mutex_manager;

pub use arc_manager::ArcManager;
pub use mutex_manager::{MutexManager, MutexRef};

/// Checks whether the function name corresponds to `std::sync::Mutex::<T>::new`.
#[inline]
pub fn is_mutex_new(function_name: &str) -> bool {
    function_name == "std::sync::Mutex::<T>::new"
}

/// Checks whether the function name corresponds to `std::sync::Mutex::<T>::lock`.
#[inline]
pub fn is_mutex_lock(function_name: &str) -> bool {
    function_name == "std::sync::Mutex::<T>::lock"
}

/// Checks whether the function name corresponds to `std::sync::Arc::<T>::new`.
#[inline]
pub fn is_arc_new(function_name: &str) -> bool {
    function_name == "std::sync::Arc::<T>::new"
}

/// Checks whether the function name corresponds to `std::ops::Deref::deref`.
#[inline]
pub fn is_deref(function_name: &str) -> bool {
    function_name == "std::ops::Deref::deref"
}

/// Checks whether the function name corresponds to `std::clone::Clone::clone`.
#[inline]
pub fn is_clone(function_name: &str) -> bool {
    function_name == "std::clone::Clone::clone"
}
