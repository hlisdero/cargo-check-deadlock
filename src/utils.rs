//! Submodule for miscellaneous utility functions.
//!
//! These functions should involve some kind of processing of the compiler types
//! which does not need additional translation data structures.

/// Extracts the definition ID of the called function from the `rustc_middle::mir::Operand`.
///
/// First obtains the type (`rustc_middle::ty::Ty`) of the operand for every possible case.
/// <https://doc.rust-lang.org/stable/nightly-rustc/rustc_middle/mir/enum.Operand.html>
///
/// Then checks that the type is a function definition (`rustc_middle::ty::TyKind::FnDef`)
/// or a closure (`rustc_middle::ty::TyKind::Closure`)
/// <https://doc.rust-lang.org/stable/nightly-rustc/rustc_middle/ty/enum.TyKind.html>
///
/// This method is used to know which function will be called as part of the `Call` MIR Terminator.
/// <https://doc.rust-lang.org/stable/nightly-rustc/rustc_middle/mir/syntax/enum.TerminatorKind.html#variant.Call>
pub fn extract_def_id_of_called_function_from_operand<'tcx>(
    operand: &rustc_middle::mir::Operand<'tcx>,
    caller_function_def_id: rustc_hir::def_id::DefId,
    tcx: rustc_middle::ty::TyCtxt<'tcx>,
) -> rustc_hir::def_id::DefId {
    let function_type = match operand {
        rustc_middle::mir::Operand::Copy(place) | rustc_middle::mir::Operand::Move(place) => {
            // Find the type through the local declarations of the caller function.
            // The `Place` (memory location) of the called function should be declared there and we can query its type.
            let body = tcx.optimized_mir(caller_function_def_id);
            let place_ty = place.ty(body, tcx);
            place_ty.ty
        }
        rustc_middle::mir::Operand::Constant(constant) => constant.ty(),
    };
    match function_type.kind() {
        rustc_middle::ty::TyKind::FnPtr(_) => {
            unimplemented!(
                "TyKind::FnPtr not implemented yet. Function pointers are present in the MIR"
            );
        }
        rustc_middle::ty::TyKind::FnDef(def_id, _)
        | rustc_middle::ty::TyKind::Closure(def_id, _) => *def_id,
        _ => {
            panic!("TyKind::FnDef, a function definition, but got: {function_type:?}")
        }
    }
}

/// Extracts the n-th argument from the arguments for the function call.
/// Returns the place corresponding to that argument.
///
/// This is also useful for obtaining the self reference for method calls.
/// For example: The call `mutex.lock()` desugars to `std::sync::Mutex::lock(&mutex)`
/// where `&self` is the first argument.
pub fn extract_nth_argument<'tcx>(
    args: &[rustc_middle::mir::Operand<'tcx>],
    index: usize,
) -> rustc_middle::mir::Place<'tcx> {
    let operand = args
        .get(index)
        .expect("BUG: Function should receive at least `index` arguments");

    match operand {
        rustc_middle::mir::Operand::Move(place) | rustc_middle::mir::Operand::Copy(place) => *place,
        rustc_middle::mir::Operand::Constant(_) => {
            unimplemented!(
                "Passing an operand of type Operand::Constant to a function is not implemented yet"
            );
        }
    }
}

/// Checks whether a given substring appears in the type of a place.
/// Uses the method `Place::ty` to get the type of the `place`.
/// It finds the type of the place through the local declarations of the caller function where it is declared.
/// <https://doc.rust-lang.org/stable/nightly-rustc/rustc_middle/mir/struct.Place.html#method.ty>
pub fn check_substring_in_place_type<'tcx>(
    place: &rustc_middle::mir::Place<'tcx>,
    expected_substring: &str,
    caller_function_def_id: rustc_hir::def_id::DefId,
    tcx: rustc_middle::ty::TyCtxt<'tcx>,
) -> bool {
    let body = tcx.optimized_mir(caller_function_def_id);
    let place_ty = place.ty(body, tcx);
    let ty_string = place_ty.ty.to_string();
    ty_string.contains(expected_substring)
}
