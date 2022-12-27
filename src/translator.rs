//! Submodule for the main translation logic.
mod function;
mod local;
mod mir_visitor;
mod mutex;
mod mutex_manager;
mod naming;
mod special_function;

use crate::stack::Stack;
use crate::translator::function::Function;
use crate::translator::local::Local;
use crate::translator::mutex_manager::MutexManager;
use crate::translator::naming::{PROGRAM_END, PROGRAM_PANIC, PROGRAM_START};
use crate::translator::special_function::SUPPORTED_SPECIAL_FUNCTIONS;
use netcrab::petri_net::{PetriNet, PlaceRef};
use rustc_middle::mir::visit::Visitor;

pub struct Translator<'tcx> {
    tcx: rustc_middle::ty::TyCtxt<'tcx>,
    err_str: Option<&'static str>,
    net: PetriNet,
    program_start: PlaceRef,
    program_end: PlaceRef,
    program_panic: PlaceRef,
    call_stack: Stack<Function>,
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

    /// Get the result of the translation, i.e. the Petri net.
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

    /// Set the error string explicitly.
    /// This is only used internally during the translation process.
    fn set_err_str(&mut self, err_str: &'static str) {
        self.err_str = Some(err_str);
    }

    /// Check whether the function name corresponds to one of the functions
    /// that needs to be translated separately, e.g, mutex functions.
    fn is_special_function(function_name: &str) -> bool {
        for name in SUPPORTED_SPECIAL_FUNCTIONS {
            if function_name == name {
                return true;
            }
        }
        false
    }

    /// Translate the source code to a Petri net.
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
        let main_return_value = Local::new();
        self.push_function_to_call_stack(
            main_function_id,
            main_return_value,
            // TODO: Arguments to the main function are not supported
            Vec::new(),
            self.program_start.clone(),
            self.program_end.clone(),
        );
        self.translate();
    }

    /// Pushes a new function frame to the call stack.
    /// The call stack is the main method to pass information between methods.
    fn push_function_to_call_stack(
        &mut self,
        function_def_id: rustc_hir::def_id::DefId,
        function_return_value: Local,
        function_args: Vec<Local>,
        start_place: PlaceRef,
        end_place: PlaceRef,
    ) {
        let function_name = self.tcx.def_path_str(function_def_id);
        let function = Function::new(
            function_def_id,
            function_name,
            function_return_value,
            function_args,
            start_place,
            end_place,
            &mut self.net,
        );
        self.call_stack.push(function);
    }

    /// Main translation loop.
    /// Translate functions from the call stack until it is empty.
    fn translate(&mut self) {
        while let Some(function) = self.call_stack.peek_mut() {
            // Translate the function to a Petri net from the MIR representation.
            let body = self.tcx.optimized_mir(function.def_id);
            // Visit the MIR body of the function using the methods of `rustc_middle::mir::visit::Visitor`.
            // <https://doc.rust-lang.org/stable/nightly-rustc/rustc_middle/mir/visit/trait.Visitor.html>
            self.visit_body(body);
            // Finished processing this function.
            self.call_stack.pop();
        }
    }
}
