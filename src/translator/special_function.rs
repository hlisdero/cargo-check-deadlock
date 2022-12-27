//! Constant definitions for functions that require special treatment
//! during the translation.
//!
//! These could be synchronization primitives or `panic!`-related primitives.

pub const SUPPORTED_SPECIAL_FUNCTIONS: [&str; 3] = [
    "std::sync::Mutex::<T>::new",
    "std::sync::Mutex::<T>::lock",
    "std::sync::Mutex::<T>::try_lock",
];
