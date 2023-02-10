//! Submodule for defining the function calls supported by the translator.

use crate::data_structures::petri_net_interface::PlaceRef;
use crate::translator::special_function::is_foreign_function;

/// A convenient typedef to pass the start place, the end place
/// and the (optional) cleanup place for a function call.
pub type FunctionPlaces = (PlaceRef, PlaceRef, Option<PlaceRef>);

/// Types of function calls that the translator supports.
pub enum FunctionCall {
    /// Call to `std::sync::Arc::<T>::new`
    /// Non-recursive call for the translation process.
    ArcNew,
    /// Call to `std::clone::Clone::clone`
    /// Non-recursive call for the translation process.
    Clone,
    /// Call to `std::sync::Condvar::new`
    /// Non-recursive call for the translation process.
    CondVarNew,
    /// Call to `std::sync::Condvar::notify_one`
    /// Non-recursive call for the translation process.
    CondVarNotifyOne,
    /// Call to `std::sync::Condvar::wait`
    /// Non-recursive call for the translation process.
    CondVarWait,
    /// Call to `std::ops::Deref::deref`
    /// Non-recursive call for the translation process.
    Deref,
    /// Call to `std::ops::DerefMut::deref_mut`
    /// Non-recursive call for the translation process.
    DerefMut,
    /// Abridged function call.
    /// Non-recursive call for the translation process.
    Foreign,
    /// MIR function call (the "default" case).
    /// Recursive call for the translation process.
    MirFunction,
    /// Call to `std::sync::Mutex::<T>::lock`.
    /// Non-recursive call for the translation process.
    MutexLock,
    /// Call to `std::sync::Mutex::<T>::new`.
    /// Non-recursive call for the translation process.
    MutexNew,
    /// Call to `std::thread::JoinHandle::<T>::join`.
    /// Non-recursive call for the translation process.
    ThreadJoin,
    /// Call to `std::thread::spawn`.
    /// Non-recursive call for the translation process.
    ThreadSpawn,
    /// Call to `std::result::Result::<T, E>::unwrap`.
    /// Non-recursive call for the translation process.
    Unwrap,
}

impl FunctionCall {
    /// Creates a new function call depending on the specific function that will be called.
    pub fn new(function_def_id: rustc_hir::def_id::DefId, tcx: rustc_middle::ty::TyCtxt) -> Self {
        let function_name = tcx.def_path_str(function_def_id);

        if let Some(function_call) = Self::is_supported_function(&function_name) {
            return function_call;
        }
        // Default case for standard and core library calls
        if is_foreign_function(function_def_id, tcx) {
            return Self::Foreign;
        }
        // Default case: A function with MIR representation
        Self::MirFunction
    }

    /// Checks if the function is one of the supported synchronization or
    /// multithreading functions.
    /// Returns the corresponding variant for the function or `None` otherwise.
    fn is_supported_function(function_name: &str) -> Option<Self> {
        match function_name {
            "std::clone::Clone::clone" => Some(Self::Clone),
            "std::ops::Deref::deref" => Some(Self::Deref),
            "std::ops::DerefMut::deref_mut" => Some(Self::DerefMut),
            "std::result::Result::<T, E>::unwrap" => Some(Self::Unwrap),
            "std::sync::Arc::<T>::new" => Some(Self::ArcNew),
            "std::sync::Condvar::new" => Some(Self::CondVarNew),
            "std::sync::Condvar::notify_one" => Some(Self::CondVarNotifyOne),
            "std::sync::Condvar::wait" => Some(Self::CondVarWait),
            "std::sync::Mutex::<T>::new" => Some(Self::MutexNew),
            "std::sync::Mutex::<T>::lock" => Some(Self::MutexLock),
            "std::thread::spawn" => Some(Self::ThreadSpawn),
            "std::thread::JoinHandle::<T>::join" => Some(Self::ThreadJoin),
            _ => None,
        }
    }
}
