//! Constant definitions for functions that require special treatment
//! during the translation.
//!
//! These could be synchronization primitives or `panic!`-related primitives.

const SUPPORTED_SPECIAL_FUNCTIONS: [&str; 3] = [
    "std::sync::Mutex::<T>::new",
    "std::sync::Mutex::<T>::lock",
    "std::sync::Mutex::<T>::try_lock",
];

/// Check whether the function name corresponds to one of the functions
/// that needs to be translated separately, e.g, mutex functions.
pub fn is_special(function_name: &str) -> bool {
    for name in SUPPORTED_SPECIAL_FUNCTIONS {
        if function_name == name {
            return true;
        }
    }
    false
}
