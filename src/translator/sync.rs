//! Submodule for implementing the translation of synchronization primitives.
//!
//! It defines which functions are supported and how to detect them.
//! It exposes only the necessary definitions to the outside modules.

mod arc_manager;
mod mutex;
mod mutex_manager;

use crate::utils::{get_place_type, is_place_ty_with_concrete_type};

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

/// Checks whether the place has type `std::sync::Mutex<T>`.
pub fn is_mutex<'tcx>(
    place: &rustc_middle::mir::Place<'tcx>,
    caller_function_def_id: rustc_hir::def_id::DefId,
    tcx: rustc_middle::ty::TyCtxt<'tcx>,
) -> bool {
    let place_ty = get_place_type(place, caller_function_def_id, tcx);
    is_place_ty_with_concrete_type(&place_ty, "std::sync::Mutex<T>")
}

/// Checks whether the place has type `&std::sync::Mutex<T>`.
pub fn is_reference_to_mutex<'tcx>(
    place: &rustc_middle::mir::Place<'tcx>,
    caller_function_def_id: rustc_hir::def_id::DefId,
    tcx: rustc_middle::ty::TyCtxt<'tcx>,
) -> bool {
    let place_ty = get_place_type(place, caller_function_def_id, tcx);
    is_place_ty_with_concrete_type(&place_ty, "&std::sync::Mutex<T>")
}

/// Detects calls to `std::sync::Arc::<T>::new` where the type of
/// the argument is `std::sync::Mutex<T>`
///
/// Returns the place of the function argument if the call has this form.
/// Returns `None` if the call does not have this form.
pub fn detect_mutex_inside_arc_new<'tcx>(
    place: &rustc_middle::mir::Place<'tcx>,
    caller_function_def_id: rustc_hir::def_id::DefId,
    tcx: rustc_middle::ty::TyCtxt<'tcx>,
) -> Option<rustc_middle::mir::Place<'tcx>> {
    let place_ty = get_place_type(place, caller_function_def_id, tcx);
    if is_place_ty_with_concrete_type(&place_ty, "std::sync::Mutex<T>") {
        return Some(*place);
    }
    None
}

/// Checks whether the place has type `&std::sync::Arc<std::sync::Mutex<T>>`.
pub fn detect_deref_arc_with_mutex<'tcx>(
    place: &rustc_middle::mir::Place<'tcx>,
    caller_function_def_id: rustc_hir::def_id::DefId,
    tcx: rustc_middle::ty::TyCtxt<'tcx>,
) -> Option<rustc_middle::mir::Place<'tcx>> {
    let place_ty = get_place_type(place, caller_function_def_id, tcx);
    if is_place_ty_with_concrete_type(&place_ty, "&std::sync::Arc<std::sync::Mutex<T>>") {
        return Some(*place);
    }
    None
}

/// Checks whether the place has type `std::sync::Arc<std::sync::Mutex<T>>`.
pub fn is_arc_with_mutex<'tcx>(
    place: &rustc_middle::mir::Place<'tcx>,
    caller_function_def_id: rustc_hir::def_id::DefId,
    tcx: rustc_middle::ty::TyCtxt<'tcx>,
) -> bool {
    let place_ty = get_place_type(place, caller_function_def_id, tcx);
    is_place_ty_with_concrete_type(&place_ty, "std::sync::Arc<std::sync::Mutex<T>>")
}

/// Detects calls to `std::clone::Clone::clone` where the type of
/// the argument is `&std::sync::Arc<std::sync::Mutex<T>>`
///
/// Returns the place of the function argument if the call has this form.
/// Returns `None` if the call does not have this form.
pub fn detect_clone_arc_with_mutex<'tcx>(
    place: &rustc_middle::mir::Place<'tcx>,
    caller_function_def_id: rustc_hir::def_id::DefId,
    tcx: rustc_middle::ty::TyCtxt<'tcx>,
) -> Option<rustc_middle::mir::Place<'tcx>> {
    let place_ty = get_place_type(place, caller_function_def_id, tcx);
    if is_place_ty_with_concrete_type(&place_ty, "&std::sync::Arc<std::sync::Mutex<T>>") {
        return Some(*place);
    }
    None
}
