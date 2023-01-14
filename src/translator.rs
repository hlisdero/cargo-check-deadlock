//! Submodule for the main translation logic.
//!
//! The source code translation takes place on the level of the Mid-level Intermediate Representation (MIR).
//! <https://rustc-dev-guide.rust-lang.org/mir/index.html>
//!
//! The `Translator` translates the MIR code of each function in approximately the order they are called.
//! For this purpose a call stack is used to represent the functions being translated, similar to how the processor
//! executes the code.
//!
//! Each MIR function consists of one or more basic blocks.
//! Each basic block consists of 0 or more statements and exactly one terminator statement.
//!
//! Functions are uniquely identified through their definition ID.
//! <https://doc.rust-lang.org/stable/nightly-rustc/rustc_hir/def_id/struct.DefId.html>
//!
//! It is possible to obtain the MIR representation for a specific function on-demand.
//! `rustc` supports a query system that computes the result and caches it automatically, which
//! saves us the work of storing everything we request from the compiler.
//! More information on the query system: <https://rustc-dev-guide.rust-lang.org/query.html>
//!
//! The naming of the places and transitions in the net is globally unique,
//! i.e. each function, block and statement receive a different label.
//! It can be configured in the `naming` submodule.

mod error_handling;
mod function_call;
mod mir_function;
mod mir_visitor;
mod naming;
mod special_function;
mod sync;
mod utils;

use crate::stack::Stack;
use crate::translator::error_handling::EMPTY_CALL_STACK;
use crate::translator::function_call::FunctionCall;
use crate::translator::mir_function::MirFunction;
use crate::translator::naming::function_foreign_call_transition_label;
use crate::translator::naming::{PROGRAM_END, PROGRAM_PANIC, PROGRAM_START};
use crate::translator::special_function::{
    diverging_function_call, foreign_function_call, is_panic, is_special, panic_function_call,
};
use crate::translator::sync::{is_mutex_function, MutexManager};
use netcrab::petri_net::{PetriNet, PlaceRef};
use rustc_middle::mir::visit::Visitor;

pub struct Translator<'tcx> {
    tcx: rustc_middle::ty::TyCtxt<'tcx>,
    err_str: Option<&'static str>,
    net: PetriNet,
    program_start: PlaceRef,
    program_end: PlaceRef,
    program_panic: PlaceRef,
    call_stack: Stack<MirFunction>,
    mutex_manager: MutexManager,
}

impl<'tcx> Translator<'tcx> {
    /// Creates a new `Translator`.
    /// Requires a global typing context `rustc_middle::ty::TyCtxt`, the main data structure of the compiler.
    /// The initial Petri net contains three places representing the program start state,
    /// the program end state and the abnormal end state after `panic!()`.
    pub fn new(tcx: rustc_middle::ty::TyCtxt<'tcx>) -> Self {
        let mut net = PetriNet::new();
        let program_panic = net.add_place(PROGRAM_PANIC);
        let program_end = net.add_place(PROGRAM_END);
        let program_start = net.add_place(PROGRAM_START);
        net.add_token(&program_start, 1).expect(
            "BUG: Adding initial token to empty PROGRAM_START place should not cause an overflow",
        );

        Self {
            tcx,
            err_str: None,
            net,
            program_start,
            program_end,
            program_panic,
            call_stack: Stack::new(),
            mutex_manager: MutexManager::new(),
        }
    }

    /// Returns the result of the translation, i.e. the Petri net.
    /// The ownership is transferred to the caller.
    ///
    /// # Errors
    ///
    /// If the translation failed, then an error is returned.
    pub fn get_result(&mut self) -> Result<PetriNet, &'static str> {
        match self.err_str {
            Some(err_str) => Err(err_str),
            // We do not want to panic here. The user should call `has_err()` first.
            None => Ok(std::mem::take(&mut self.net)),
        }
    }

    /// Sets the error string explicitly.
    /// This is only used internally during the translation process.
    fn set_err_str(&mut self, err_str: &'static str) {
        self.err_str = Some(err_str);
    }

    /// Translates the source code to a Petri net.
    ///
    /// # Errors
    ///
    /// If the translation fails due to a recoverable error, then an error message is set.
    ///
    /// # Panics
    ///
    /// If the translation fails due to an unsupported feature present in the code, then the function panics.
    pub fn run(&mut self) {
        let Some((main_function_id, _)) = self.tcx.entry_fn(()) else {
            self.set_err_str("No main function found in the given source code");
            return;
        };
        self.push_function_to_call_stack(
            main_function_id,
            self.program_start.clone(),
            self.program_end.clone(),
        );
        self.translate_top_call_stack();
    }

    /// Pushes a new function frame to the call stack.
    /// The call stack is the preferred way to pass information between `Translator` methods.
    fn push_function_to_call_stack(
        &mut self,
        function_def_id: rustc_hir::def_id::DefId,
        start_place: PlaceRef,
        end_place: PlaceRef,
    ) {
        let function = MirFunction::new(function_def_id, start_place, end_place, &mut self.tcx);
        self.call_stack.push(function);
    }

    /// Main translation loop.
    /// Translates the function from the top of the call stack.
    /// Inside the MIR Visitor, when a call to another function happens, this method will be called again
    /// to jump to the new function. Eventually a "leaf function" will be reached, the functions will exit and the
    /// elements from the stack will be popped in order.
    fn translate_top_call_stack(&mut self) {
        let function = self.call_stack.peek_mut().expect(EMPTY_CALL_STACK);
        // Obtain the MIR representation of the function.
        let body = self.tcx.optimized_mir(function.def_id);
        // Visit the MIR body of the function using the methods of `rustc_middle::mir::visit::Visitor`.
        // <https://doc.rust-lang.org/stable/nightly-rustc/rustc_middle/mir/visit/trait.Visitor.html>
        self.visit_body(body);
        // Finished processing this function.
        self.call_stack.pop();
    }

    /// Extracts the function call ID from the `rustc_middle::mir::Operand`.
    ///
    /// First obtains the type (`rustc_middle::ty::Ty`) of the operand for every possible case.
    /// <https://doc.rust-lang.org/stable/nightly-rustc/rustc_middle/mir/enum.Operand.html>
    ///
    /// Then checks that the type is a function definition (`rustc_middle::ty::TyKind::FnDef`)
    /// <https://doc.rust-lang.org/stable/nightly-rustc/rustc_middle/ty/enum.TyKind.html>
    ///
    /// This method is used to know which function will be called as part of the `Call` MIR Terminator.
    /// <https://doc.rust-lang.org/stable/nightly-rustc/rustc_middle/mir/syntax/enum.TerminatorKind.html#variant.Call>
    fn extract_def_id_of_called_function_from_operand(
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

    /// Checks whether the function with the given function name
    /// and given `DefId` should be treated as a foreign function call.
    fn is_foreign_function_call(
        &self,
        function_def_id: rustc_hir::def_id::DefId,
        function_name: &str,
    ) -> bool {
        self.tcx.is_foreign_item(function_def_id)
            || !self.tcx.is_mir_available(function_def_id)
            || is_special(function_name)
    }

    /// Prepares the function call depending on the type of function.
    /// The return `FunctionCall` enum has all the information required for the function call.
    ///
    /// This is the handler for the enum variant `TerminatorKind::Call` in the MIR Visitor.
    /// <https://doc.rust-lang.org/stable/nightly-rustc/rustc_middle/mir/enum.TerminatorKind.html#variant.Call>
    fn prepare_function_call(
        &mut self,
        func: &rustc_middle::mir::Operand<'tcx>,
        args: Vec<rustc_middle::mir::Operand<'tcx>>,
        destination: rustc_middle::mir::Place<'tcx>,
        target: Option<rustc_middle::mir::BasicBlock>,
        cleanup: Option<rustc_middle::mir::BasicBlock>,
    ) -> FunctionCall<'tcx> {
        let current_function = self.call_stack.peek_mut().expect(EMPTY_CALL_STACK);
        let function_def_id = Self::extract_def_id_of_called_function_from_operand(
            func,
            current_function.def_id,
            self.tcx,
        );
        let function_name = self.tcx.def_path_str(function_def_id);

        if is_panic(&function_name) {
            let start_place = current_function.get_start_place_for_function_call();
            return FunctionCall::Panic {
                function_name: current_function.name.clone(),
                start_place,
            };
        }

        let Some(return_block) = target else {
            let start_place = current_function.get_start_place_for_function_call();
            return FunctionCall::Diverging { function_name, start_place };
        };

        let (start_place, end_place, cleanup_place) =
            current_function.get_place_refs_for_function_call(return_block, cleanup, &mut self.net);

        if self.is_foreign_function_call(function_def_id, &function_name) {
            return FunctionCall::Foreign {
                function_name,
                start_place,
                end_place,
                cleanup_place,
            };
        }

        if is_mutex_function(&function_name) {
            return FunctionCall::Mutex {
                function_name,
                args,
                destination,
                start_place,
                end_place,
                cleanup_place,
            };
        }
        // Default case: A function with MIR representation
        FunctionCall::MirFunction {
            function_def_id,
            start_place,
            end_place,
        }
    }

    /// Jumps from the current function on the top of the stack
    /// to a new function called inside the current function.
    ///
    /// Translates functions in a shortened way in the following cases:
    /// - Foreign items i.e., linked via extern { ... }).
    /// - Functions that do not have a MIR representation.
    /// - Functions in a list of special functions defined by `translator::special_function::SUPPORTED_SPECIAL_FUNCTIONS`.
    /// - Functions that call a mutex synchronization primitive such as `std::sync::Mutex::lock`.
    ///
    /// Diverging functions are handled here too. They are modelled as a dead end in the net.
    pub fn call_function(&mut self, function_call: FunctionCall) {
        match function_call {
            FunctionCall::MirFunction {
                function_def_id,
                start_place,
                end_place,
            } => {
                self.push_function_to_call_stack(function_def_id, start_place, end_place);
                self.translate_top_call_stack();
            }
            FunctionCall::Diverging {
                function_name,
                start_place,
            } => diverging_function_call(&start_place, &function_name, &mut self.net),
            FunctionCall::Foreign {
                function_name,
                start_place,
                end_place,
                cleanup_place,
            } => {
                let transition_label = &function_foreign_call_transition_label(&function_name);
                foreign_function_call(
                    &start_place,
                    &end_place,
                    cleanup_place,
                    transition_label,
                    &mut self.net,
                );
            }
            FunctionCall::Mutex {
                function_name,
                args,
                destination,
                start_place,
                end_place,
                cleanup_place,
            } => {
                let transition_function_call = self.mutex_manager.translate_function_call(
                    &function_name,
                    &start_place,
                    &end_place,
                    cleanup_place,
                    &mut self.net,
                );
                self.mutex_manager.translate_function_side_effects(
                    &function_name,
                    &args,
                    destination,
                    &transition_function_call,
                    &mut self.net,
                );
            }
            FunctionCall::Panic {
                function_name,
                start_place,
            } => panic_function_call(
                &start_place,
                &self.program_panic,
                &function_name,
                &mut self.net,
            ),
        }
    }
}
