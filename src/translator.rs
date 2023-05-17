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
//!
//! A `HashMapCounter` keeps track of how many time each function name has been seen so far.
//! After every call the counter for the corresponding function is incremented.

mod function;
mod function_call_handler;
mod mir_function;
mod mir_visitor;
mod special_function;
mod sync;

use crate::data_structures::hash_map_counter::HashMapCounter;
use crate::data_structures::petri_net_interface::{PetriNet, PlaceRef};
use crate::data_structures::stack::Stack;
use crate::naming::program::{PROGRAM_END, PROGRAM_PANIC, PROGRAM_START};
use crate::utils::extract_def_id_of_called_function_from_operand;
use function::Places;
use log::info;
use mir_function::MirFunction;
use rustc_middle::mir::visit::Visitor;
use special_function::{call_diverging_function, call_panic_function, is_panic_function};
use sync::{CondvarManager, MutexManager, ThreadManager};

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
    function_counter: HashMapCounter,
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
            function_counter: HashMapCounter::new(),
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
        info!("Pushed main function to the translation call stack");
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
        let function_name = self.tcx.def_path_str(function_def_id);
        let function = MirFunction::new(function_def_id, function_name, start_place, end_place);
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
    /// - Functions for condition variables: `std::sync::Condvar::new`, `std::sync::Condvar::wait` and `std::sync::Condvar::notify_one`.
    /// - Functions from the Rust standard library or the Rust core library.
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
        let start_place = current_function.get_start_place_for_function_call();
        info!("Encountered function call: {function_name}");

        // Depending on whether a return or a unwind for the function are present,
        // we have different possibilities for the function call end place and the (optional) cleanup place.
        let places = match (target, unwind) {
            (Some(return_block), rustc_middle::mir::UnwindAction::Continue) => {
                // MIR function or foreign function calls without a cleanup block.
                let end_place =
                    current_function.get_end_place_for_function_call(return_block, &mut self.net);
                Places::Basic {
                    start_place,
                    end_place,
                }
            }
            (Some(return_block), rustc_middle::mir::UnwindAction::Cleanup(cleanup_block)) => {
                // The usual foreign function call case.
                let end_place =
                    current_function.get_end_place_for_function_call(return_block, &mut self.net);
                let cleanup_place =
                    current_function.get_end_place_for_function_call(cleanup_block, &mut self.net);
                Places::WithCleanup {
                    start_place,
                    end_place,
                    cleanup_place,
                }
            }
            (Some(return_block), rustc_middle::mir::UnwindAction::Terminate) => {
                // Specific foreign function calls that terminate the program (abort).
                let end_place =
                    current_function.get_end_place_for_function_call(return_block, &mut self.net);
                // Connect cleanup to panic place
                Places::WithCleanup {
                    start_place,
                    end_place,
                    cleanup_place: self.program_panic.clone(),
                }
            }
            (None, rustc_middle::mir::UnwindAction::Terminate) => {
                // Foreign function calls that simply terminate the program.
                Places::Basic {
                    start_place,
                    end_place: self.program_panic.clone(),
                }
            }
            (None, rustc_middle::mir::UnwindAction::Cleanup(cleanup_block)) => {
                // A very special case seen in functions like `std::process::exit`
                // where the return block is actually expressed as a cleanup.
                // This needs to be modelled differently than a diverging function.
                let end_place =
                    current_function.get_end_place_for_function_call(cleanup_block, &mut self.net);
                Places::Basic {
                    start_place,
                    end_place,
                }
            }
            (None, rustc_middle::mir::UnwindAction::Continue) => {
                // Call to a function which does not return (Return type: -> !).
                // Non-recursive call for the translation process.
                // `panic!`-related functions are a special case of this.
                if is_panic_function(&function_name) {
                    call_panic_function(
                        &start_place,
                        &self.program_panic,
                        &current_function.name,
                        &mut self.net,
                    );
                } else {
                    call_diverging_function(&start_place, &function_name, &mut self.net);
                }
                return;
            }
            (Some(return_block), rustc_middle::mir::UnwindAction::Unreachable) => {
                // Support the unreachable case simply by matching the cleanup place to the program end place.
                // This is a compromise solution to avoid polluting the panic state with these extraneous states
                // that are actually not reachable during execution.
                let end_place =
                    current_function.get_end_place_for_function_call(return_block, &mut self.net);
                // Connect cleanup to program end place.
                Places::WithCleanup {
                    start_place,
                    end_place,
                    cleanup_place: self.program_end.clone(),
                }
            }
            (None, rustc_middle::mir::UnwindAction::Unreachable) => {
                // Support the unreachable case simply by matching the cleanup place to the program end place.
                // This is a compromise solution to avoid polluting the panic state with these extraneous states
                // that are actually not reachable during execution.
                Places::Basic {
                    start_place,
                    end_place: self.program_end.clone(),
                }
            }
        };

        self.start_function_call(function_def_id, &function_name, args, destination, places);
    }

    /// Handler for the enum variant `TerminatorKind::Drop` in the MIR Visitor.
    /// <https://doc.rust-lang.org/stable/nightly-rustc/rustc_middle/mir/enum.TerminatorKind.html#variant.Drop>
    fn drop_terminator(
        &mut self,
        place: rustc_middle::mir::Place<'tcx>,
        target: rustc_middle::mir::BasicBlock,
        unwind: rustc_middle::mir::UnwindAction,
    ) {
        let function = self.call_stack.peek_mut();

        let (transition, cleanup_transition) = match unwind {
            rustc_middle::mir::UnwindAction::Cleanup(cleanup) => {
                function.drop(target, Some(cleanup), &mut self.net)
            }
            // Do NOT model the `Terminate` case.
            // It is not relevant for deadlock detection and makes the Petri nets unnecessarily bigger.
            rustc_middle::mir::UnwindAction::Continue
            | rustc_middle::mir::UnwindAction::Terminate => {
                function.drop(target, None, &mut self.net)
            }
            rustc_middle::mir::UnwindAction::Unreachable => {
                function.unreachable(&self.program_end, &mut self.net);
                function.drop(target, None, &mut self.net)
            }
        };
        self.mutex_manager.handle_mutex_guard_drop(
            place,
            &transition,
            &function.memory,
            &mut self.net,
        );
        if let Some(cleanup_transition) = cleanup_transition {
            self.mutex_manager.handle_mutex_guard_drop(
                place,
                &cleanup_transition,
                &function.memory,
                &mut self.net,
            );
        }
    }

    /// Handler for the enum variant `TerminatorKind::Assert` in the MIR Visitor.
    /// <https://doc.rust-lang.org/stable/nightly-rustc/rustc_middle/mir/enum.TerminatorKind.html#variant.Assert>
    fn assert_terminator(
        &mut self,
        target: rustc_middle::mir::BasicBlock,
        unwind: rustc_middle::mir::UnwindAction,
    ) {
        let function = self.call_stack.peek_mut();

        match unwind {
            rustc_middle::mir::UnwindAction::Cleanup(cleanup) => {
                function.assert(target, Some(cleanup), &mut self.net);
            }
            // Do NOT model the `Terminate` case.
            // It is not relevant for deadlock detection and makes the Petri nets unnecessarily bigger.
            rustc_middle::mir::UnwindAction::Continue
            | rustc_middle::mir::UnwindAction::Terminate => {
                function.assert(target, None, &mut self.net);
            }
            rustc_middle::mir::UnwindAction::Unreachable => {
                function.assert(target, None, &mut self.net);
                function.unreachable(&self.program_end, &mut self.net);
            }
        }
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
            info!("Pushed thread function to the translation call stack");
            let new_function = self.call_stack.peek_mut();
            info!("Moving sync variables to the thread function...");
            thread.move_sync_variables(&mut new_function.memory, self.tcx);
            self.translate_top_call_stack();
            info!("Finished translating thread {}", thread.index);
        }
    }
}
