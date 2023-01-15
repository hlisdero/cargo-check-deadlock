//! Submodule for miscellaneous utility functions.
//!
//! These functions should involve some kind of processing of the compiler types
//! which does not need additional translation data structures.

/// Convert the `Place` directly to a `Local`.
/// <https://doc.rust-lang.org/stable/nightly-rustc/rustc_middle/mir/syntax/struct.Place.html#method.as_local>
///
/// # Panics
///
/// If `place` is not a simple local variable without projections, then the function panics.
/// <https://rustc-dev-guide.rust-lang.org/mir/index.html#mir-data-types>
pub fn place_to_local(place: &rustc_middle::mir::Place) -> rustc_middle::mir::Local {
    place
        .as_local()
        .expect("BUG: The place should be a local variable with no projections")
}

/// Extracts the definition ID of the called function from the `rustc_middle::mir::Operand`.
///
/// First obtains the type (`rustc_middle::ty::Ty`) of the operand for every possible case.
/// <https://doc.rust-lang.org/stable/nightly-rustc/rustc_middle/mir/enum.Operand.html>
///
/// Then checks that the type is a function definition (`rustc_middle::ty::TyKind::FnDef`)
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
            let place_ty = place.ty(&body.local_decls, tcx);
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
        rustc_middle::ty::TyKind::FnDef(def_id, _) => *def_id,
        _ => {
            panic!("TyKind::FnDef, a function definition, but got: {function_type:?}")
        }
    }
}

/// Extracts the self reference from the function arguments.
/// For example: The call `mutex.lock()` desugars to `std::sync::Mutex::lock(&mutex)`
/// where `&self` is the first argument.
pub fn extract_self_reference_from_arguments_for_function_call<'tcx>(
    args: &[rustc_middle::mir::Operand<'tcx>],
) -> rustc_middle::mir::Place<'tcx> {
    let rustc_middle::mir::Operand::Move(self_ref) = args.get(0)
            .expect("BUG: Function should receive a reference to self as the 0-th function argument") else { 
                panic!("BUG: The self reference should be passed by moving");
        };
    *self_ref
}
