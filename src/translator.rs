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

mod function_call;
mod function_call_handler;
mod function_counter;
mod mir_function;
mod mir_visitor;
mod special_function;
mod sync;

use crate::data_structures::petri_net_interface::{PetriNet, PlaceRef};
use crate::data_structures::stack::Stack;
use crate::naming::program::{PROGRAM_END, PROGRAM_PANIC, PROGRAM_START};
use crate::utils::extract_def_id_of_called_function_from_operand;
use function_call::FunctionCall;
use function_counter::FunctionCounter;
use log::info;
use mir_function::MirFunction;
use rustc_middle::mir::visit::Visitor;
use special_function::{call_diverging_function, call_panic_function, is_panic_function};
use sync::{
    handle_aggregate_assignment, link_if_sync_variable, CondvarManager, MutexManager, ThreadManager,
};

/// Translator error message when no main function is found in the source code.
pub const ERR_NO_MAIN_FUNCTION_FOUND: &str = "ERROR: No main function found in the source code";

pub struct Translator<'tcx> {
    tcx: rustc_middle::ty::TyCtxt<'tcx>,
    err_str: Option<&'static str>,
    net: PetriNet,
    program_start: PlaceRef,
    program_end: PlaceRef,
    program_panic: PlaceRef,
    call_stack: Stack<MirFunction<'tcx>>,
    function_counter: FunctionCounter,
    condvar_manager: CondvarManager,
    mutex_manager: MutexManager,
    thread_manager: ThreadManager,
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
            function_counter: FunctionCounter::new(),
            condvar_manager: CondvarManager::new(),
            mutex_manager: MutexManager::new(),
            thread_manager: ThreadManager::new(),
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
            None => Ok(std::mem::take(&mut self.net)),
        }
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
            self.err_str = Some(ERR_NO_MAIN_FUNCTION_FOUND);
            return;
        };
        self.push_function_to_call_stack(
            main_function_id,
            self.program_start.clone(),
            self.program_end.clone(),
        );
        info!("Pushed main function to the translation callstack");
        self.translate_top_call_stack();
        info!("Finished translating the main thread");
        self.translate_threads();
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
        let function = self.call_stack.peek();
        // Obtain the MIR representation of the function.
        let body = self.tcx.optimized_mir(function.def_id);
        // Visit the MIR body of the function using the methods of `rustc_middle::mir::visit::Visitor`.
        // <https://doc.rust-lang.org/stable/nightly-rustc/rustc_middle/mir/visit/trait.Visitor.html>
        self.visit_body(body);
        // Finished processing this function.
        self.call_stack.pop();
    }

    /// Jumps from the current function on the top of the stack
    /// to a new function called inside the current function.
    ///
    /// Translates functions in a shortened way in the following cases:
    /// - Foreign functions i.e., linked via extern { ... }).
    /// - Functions that do not have a MIR representation.
    /// - Functions that do not return (diverging functions).
    /// - Functions that represent a `panic` i.e., functions that starts an unwind of the stack.
    /// - Functions for mutexes: `std::sync::Mutex::new` and `std::sync::Mutex::lock`.
    /// - Functions for threads: `std::thread::spawn` and `std::thread::JoinHandle::<T>::join`.
    /// - Functions for reference counters: `std::sync::Arc::new`, `std::sync::Arc::clone` and `std::sync::Arc::deref`.
    /// - Functions for condition variables: `std::sync::Condvar::new`, `std::sync::Condvar::wait` and `std::sync::Condvar::notify_one`.
    /// - Functions that call the Rust standard library or the Rust core library.
    ///
    /// This is the handler for the enum variant `TerminatorKind::Call` in the MIR Visitor.
    /// <https://doc.rust-lang.org/stable/nightly-rustc/rustc_middle/mir/enum.TerminatorKind.html#variant.Call>
    fn call_function(
        &mut self,
        func: &rustc_middle::mir::Operand<'tcx>,
        args: &[rustc_middle::mir::Operand<'tcx>],
        destination: rustc_middle::mir::Place<'tcx>,
        target: Option<rustc_middle::mir::BasicBlock>,
        unwind: rustc_middle::mir::UnwindAction,
    ) {
        let current_function = self.call_stack.peek_mut();
        let function_def_id =
            extract_def_id_of_called_function_from_operand(func, current_function.def_id, self.tcx);
        let function_name = self.tcx.def_path_str(function_def_id);
        info!("Encountered function call: {function_name}");

        if is_panic_function(&function_name) {
            // Function call which starts an abnormal termination of the program.
            // Non-recursive call for the translation process.
            let start_place = current_function.get_start_place_for_function_call();
            call_panic_function(
                &start_place,
                &self.program_panic,
                &current_function.name,
                &mut self.net,
            );
            return;
        }

        let Some(return_block) = target else {
            // Call to a function which does not return (Return type: -> !).
            // Non-recursive call for the translation process.
            let start_place = current_function.get_start_place_for_function_call();
            call_diverging_function(&start_place, &function_name, &mut self.net);
            return;
        };

        let place_refs_for_function_call =
            current_function.get_place_refs_for_function_call(return_block, unwind, &mut self.net);

        let function_call = FunctionCall::new(function_def_id, self.tcx);
        self.start_function_call(
            &function_call,
            function_def_id,
            args,
            destination,
            place_refs_for_function_call,
        );
    }

    /// Main translation loop for the threads.
    /// Gets a thread from the thread manager and translates it.
    /// If mutexes were passed to the thread, move them to the memory of the thread function.
    /// Replaces the program panic place with the thread's end place, since abnormal thread
    /// termination does not affect the main thread.
    fn translate_threads(&mut self) {
        while let Some(mut thread) = self.thread_manager.pop_thread() {
            info!("Starting translating thread {}", thread.index);
            let (thread_function_def_id, thread_start_place, thread_end_place) =
                thread.prepare_for_translation(&mut self.net);
            // Replace the panic place so that unwind transitions and similar point to the thread's end place.
            self.program_panic = thread_end_place.clone();

            self.push_function_to_call_stack(
                thread_function_def_id,
                thread_start_place,
                thread_end_place,
            );
            info!("Pushed thread function to the translation callstack");
            let new_function = self.call_stack.peek_mut();
            info!("Moving sync variables to the thread function...");
            thread.move_sync_variables(&mut new_function.memory, self.tcx);
            self.translate_top_call_stack();
        }
    }

    /// Handles MIR assignments of the form: `_X = _Y`
    /// This is the handler for the enum variant `rustc_middle::mir::Rvalue::Use` in the MIR Visitor
    /// when the argument is of the type `rustc_middle::mir::Operand::Copy`.
    /// <https://doc.rust-lang.org/stable/nightly-rustc/rustc_middle/mir/enum.Rvalue.html#variant.Use>
    /// <https://doc.rust-lang.org/stable/nightly-rustc/rustc_middle/mir/syntax/enum.Operand.html#variant.Copy>
    fn handle_use_copy_assignment(
        &mut self,
        place: &rustc_middle::mir::Place<'tcx>,
        rhs: &rustc_middle::mir::Place<'tcx>,
    ) {
        let function = self.call_stack.peek_mut();
        link_if_sync_variable(place, rhs, &mut function.memory, function.def_id, self.tcx);
    }

    /// Handles MIR assignments of the form: `_X = move _Y`
    /// This is the handler for the enum variant `rustc_middle::mir::Rvalue::Use` in the MIR Visitor
    /// when the argument is of the type `rustc_middle::mir::Operand::Move`.
    /// <https://doc.rust-lang.org/stable/nightly-rustc/rustc_middle/mir/enum.Rvalue.html#variant.Use>
    /// <https://doc.rust-lang.org/stable/nightly-rustc/rustc_middle/mir/syntax/enum.Operand.html#variant.Move>
    fn handle_use_move_assignment(
        &mut self,
        place: &rustc_middle::mir::Place<'tcx>,
        rhs: &rustc_middle::mir::Place<'tcx>,
    ) {
        let function = self.call_stack.peek_mut();
        link_if_sync_variable(place, rhs, &mut function.memory, function.def_id, self.tcx);
    }

    /// Handles MIR assignments of the form: `_X = &_Y`
    /// This is the handler for the enum variant `rustc_middle::mir::Rvalue::Ref` in the MIR Visitor.
    /// <https://doc.rust-lang.org/stable/nightly-rustc/rustc_middle/mir/enum.Rvalue.html#variant.Ref>
    fn handle_ref_assignment(
        &mut self,
        place: &rustc_middle::mir::Place<'tcx>,
        rhs: &rustc_middle::mir::Place<'tcx>,
    ) {
        let function = self.call_stack.peek_mut();
        link_if_sync_variable(place, rhs, &mut function.memory, function.def_id, self.tcx);
    }

    /// Handles MIR assignments of the form: `_X = { copy_data: move _Y }`.
    /// This is the handler for the enum variant `rustc_middle::mir::Rvalue::Aggregate` in the MIR Visitor.
    /// <https://doc.rust-lang.org/stable/nightly-rustc/rustc_middle/mir/enum.Rvalue.html#variant.Aggregate>
    fn handle_aggregate_assignment(
        &mut self,
        place: &rustc_middle::mir::Place<'tcx>,
        operands: &Vec<rustc_middle::mir::Operand<'tcx>>,
    ) {
        let function = self.call_stack.peek_mut();
        handle_aggregate_assignment(
            place,
            operands,
            &mut function.memory,
            function.def_id,
            self.tcx,
        );
    }
}
