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
mod mir_function;
mod mir_visitor;
mod special_function;
mod sync;

use log::{debug, info};
use rustc_middle::mir::visit::Visitor;
use rustc_middle::mir::UnwindAction;
use std::collections::{BinaryHeap, VecDeque};
use std::rc::Rc;

use crate::data_structures::hash_map_counter::HashMapCounter;
use crate::data_structures::petri_net_interface::{connect_places, PetriNet, PlaceRef};
use crate::data_structures::stack::Stack;
use crate::naming::function::{indexed_mir_function_cleanup_label, indexed_mir_function_name};
use crate::naming::{PROGRAM_END, PROGRAM_PANIC, PROGRAM_START};
use crate::translator::mir_function::memory::Single;
use crate::utils::{extract_def_id_of_called_function_from_operand, extract_nth_argument_as_place};
use function::{Places, PostprocessingTask, Transitions};
use mir_function::memory::{MutexRef, Value};
use mir_function::MirFunction;
use special_function::{
    call_diverging_function, call_foreign_function, call_panic_function, is_foreign_function,
    is_panic_function,
};
use sync::thread::Thread;
use sync::{check_if_mutex_variable, mutex};

/// The central data structure and coordinator for the translation.
pub struct Translator<'tcx> {
    /// The global typing context that enables interaction with `rustc` during the translation.
    tcx: rustc_middle::ty::TyCtxt<'tcx>,
    /// The Petri net model of the program being translated.
    net: PetriNet,
    /// The place in the Petri net that models the program initial state.
    program_start: PlaceRef,
    /// The place in the Petri net that models a normal program end state, regardless of exit code.
    program_end: PlaceRef,
    /// The place in the Petri net that models the `panic!` end state.
    program_panic: PlaceRef,
    /// The call stack of user-defined functions with a MIR representation.
    /// Other functions are translated in an abbreviated form.
    call_stack: Stack<MirFunction>,
    /// A counter that keeps track how many times each function was called so far.
    /// Functions are identified by their name.
    function_counter: HashMapCounter,
    /// A vector of threads detected in the code.
    /// They are translated in order after the main thread.
    threads: VecDeque<Rc<Thread>>,
    /// Translation tasks performed after all threads have been translated.
    /// These tasks usually require to make changes to the final Petri net.
    postprocessing: BinaryHeap<PostprocessingTask>,
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
            net,
            program_start,
            program_end,
            program_panic,
            call_stack: Stack::new(),
            function_counter: HashMapCounter::new(),
            threads: VecDeque::new(),
            postprocessing: BinaryHeap::new(),
        }
    }

    /// Returns the result of the translation, i.e. the Petri net.
    /// The ownership is transferred to the caller.
    pub fn get_result(&mut self) -> PetriNet {
        std::mem::take(&mut self.net)
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
        let (main_function_id, _) = self
            .tcx
            .entry_fn(())
            .expect("ERROR: No main function found in the source code");
        let function_name = self.tcx.def_path_str(main_function_id);
        let function = MirFunction::new(
            main_function_id,
            function_name,
            self.program_start.clone(),
            self.program_end.clone(),
        );
        self.call_stack.push(function);
        info!("Pushed main function to the translation call stack");
        self.translate_top_call_stack();
        info!("Finished translating the main thread");
        self.translate_threads();
        info!("Running translation postprocessing...");
        self.translation_postprocessing();
    }

    /// Main translation loop for the threads.
    /// Iterate over the threads found and translate them.
    /// If sync variables were passed to the thread, move them to the memory of the thread function.
    /// Replaces the program panic place with the thread's end place
    /// since abnormal thread termination does not affect the main thread.
    fn translate_threads(&mut self) {
        while let Some(thread) = self.threads.pop_front() {
            let index = thread.index;

            info!("Starting translating thread {index}");
            // Replace the panic place so that unwind transitions and similar point to the thread's end place.
            self.program_panic = thread.clone_end_place();
            thread.create_arcs_for_transitions(&mut self.net);
            let mir_function = Rc::into_inner(thread)
                .expect("BUG: There should be only one reference to the thread by the time we translate it")
                .take_mir_function();
            self.call_stack.push(mir_function);
            info!("Pushed thread function to the translation call stack");

            self.translate_top_call_stack();
            info!("Finished translating thread {index}");
        }
    }

    /// Run the postprocessing tasks.
    /// These tasks require knowledge of the whole Petri net.
    /// For example: Adding arcs or places after all threads have been translated.
    fn translation_postprocessing(&mut self) {
        let mut mutexes: Vec<MutexRef> = Vec::new();
        while let Some(task) = self.postprocessing.pop() {
            match task {
                PostprocessingTask::LinkMutexToCondvar {
                    index,
                    start_place,
                    end_place,
                    wait_start,
                    ..
                } => {
                    for mutex_ref in &mutexes {
                        mutex_ref.link_to_condvar(
                            index,
                            &start_place,
                            &end_place,
                            &wait_start,
                            &mut self.net,
                        );
                    }
                }
                PostprocessingTask::NewMutex { mutex_ref, .. } => {
                    mutexes.push(mutex_ref);
                }
            }
        }
    }

    /// Main translation loop.
    /// Translates the function from the top of the call stack.
    /// Inside the MIR Visitor, when a call to another function happens, this method will be called again
    /// to jump to the new function. Eventually a "leaf function" will be reached, the functions will exit and the
    /// elements from the stack will be popped in order.
    fn translate_top_call_stack(&mut self) -> Option<Value> {
        let function = self.call_stack.peek();
        // Obtain the MIR representation of the function.
        let body = self.tcx.optimized_mir(function.def_id);
        // Visit the MIR body of the function using the methods of `rustc_middle::mir::visit::Visitor`.
        // <https://doc.rust-lang.org/stable/nightly-rustc/rustc_middle/mir/visit/trait.Visitor.html>
        self.visit_body(body);
        let function = self.call_stack.peek();
        let value = function.memory.get_return_value();
        // Finished processing this function.
        self.call_stack.pop();
        value
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
        args: &[rustc_span::source_map::Spanned<rustc_middle::mir::Operand<'tcx>>],
        destination: rustc_middle::mir::Place<'tcx>,
        target: Option<rustc_middle::mir::BasicBlock>,
        unwind: UnwindAction,
    ) -> Option<Value> {
        let current_function = self.call_stack.peek_mut();
        let function_def_id =
            extract_def_id_of_called_function_from_operand(func, current_function.def_id, self.tcx);
        let function_name = self.tcx.def_path_str(function_def_id);
        let start_place = current_function.get_start_place_for_function_call();
        info!("Encountered function call: {function_name}");

        // Depending on whether a return or a unwind for the function are present,
        // we have different possibilities for the function call end place and the (optional) cleanup place.
        let places = match (target, unwind) {
            (Some(return_block), UnwindAction::Continue) => {
                // MIR function or foreign function calls without a cleanup block.
                let end_place =
                    current_function.get_end_place_for_function_call(return_block, &mut self.net);
                Places::Basic {
                    start_place,
                    end_place,
                }
            }
            (Some(return_block), UnwindAction::Cleanup(cleanup_block)) => {
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
            (Some(return_block), UnwindAction::Terminate(..)) => {
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
            (None, UnwindAction::Terminate(..)) => {
                // Foreign function calls that simply terminate the program.
                Places::Basic {
                    start_place,
                    end_place: self.program_panic.clone(),
                }
            }
            (None, UnwindAction::Cleanup(cleanup_block)) => {
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
            (None, UnwindAction::Continue) => {
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
                return None; // Diverging function do not return a value
            }
            (Some(return_block), UnwindAction::Unreachable) => {
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
            (None, UnwindAction::Unreachable) => {
                // Support the unreachable case simply by matching the cleanup place to the program end place.
                // This is a compromise solution to avoid polluting the panic state with these extraneous states
                // that are actually not reachable during execution.
                Places::Basic {
                    start_place,
                    end_place: self.program_end.clone(),
                }
            }
        };

        let return_value = self.translate_function_call(
            function_def_id,
            &function_name,
            args,
            destination,
            places,
        );
        self.function_counter.increment(&function_name);
        return_value
    }

    /// Finds and calls the corresponding handler for the function call.
    /// Checks if the function is one of the
    /// supported synchronization or multithreading functions,
    /// then if the function is a foreign function call and
    /// lastly handle the standard MIR function case.
    pub fn translate_function_call(
        &mut self,
        function_def_id: rustc_hir::def_id::DefId,
        function_name: &str,
        args: &[rustc_span::source_map::Spanned<rustc_middle::mir::Operand<'tcx>>],
        destination: rustc_middle::mir::Place<'tcx>,
        places: Places,
    ) -> Option<Value> {
        // Special cases
        if function_name == "std::mem::drop" {
            return self.call_mem_drop(function_name, args, places);
        }
        if (function_name == "std::ops::Deref::deref"
            || function_name == "std::ops::DerefMut::deref_mut")
            && self.is_self_ref_mutex(function_name, args)
        {
            return self.call_deref_mutex(function_name, args, places);
        }
        if function_name == "std::result::Result::<T, E>::unwrap"
            && self.is_self_ref_mutex(function_name, args)
        {
            return self.call_unwrap_mutex(function_name, args, places);
        }
        if function_name == "std::thread::spawn" {
            let return_value = self.call_thread_spawn(function_name, args, destination, places);
            return Some(return_value);
        }
        // Sync or multithreading function
        if sync::is_supported_function(function_name) {
            // Index for transition and place labels
            let index = self.function_counter.get_count(function_name);
            // A reference to the memory of the current function
            let current_function = self.call_stack.peek_mut();
            let memory = &mut current_function.memory;
            // A reference to the Petri net to add transitions and places
            let net = &mut self.net;
            if let Some(task) =
                sync::call_function(function_name, index, args, destination, places, net, memory)
            {
                self.postprocessing.push(task);
            }
            return None;
        }
        // Default case for standard and core library calls
        if is_foreign_function(function_def_id, function_name, self.tcx) {
            let index = self.function_counter.get_count(function_name);
            call_foreign_function(function_name, index, places, &mut self.net);
            return self.get_linked_value_in_first_argument(args);
        }
        // Default case: A function with MIR representation
        self.call_mir_function(function_def_id, function_name, args, places)
    }

    /// Returns the value linked to the place pointed to by the first operand in the function call
    /// if the place is linked to a value. Otherwise, returns `None`.
    /// This handles several cases where the sync variable is just "passed" through a function that receives a single parameter.
    fn get_linked_value_in_first_argument(
        &mut self,
        args: &[rustc_span::source_map::Spanned<rustc_middle::mir::Operand<'tcx>>],
    ) -> Option<Value> {
        let Some(first_argument) = extract_nth_argument_as_place(args, 0) else {
            return None; // Nothing to return: Either the first argument is not present or it is a constant.
        };
        let function = self.call_stack.peek_mut();
        function.memory.get_linked_value_or_none(&first_argument)
    }

    /// Checks whether the first argument (the self reference) is a mutex or a mutex guard.
    fn is_self_ref_mutex(
        &self,
        function_name: &str,
        args: &[rustc_span::source_map::Spanned<rustc_middle::mir::Operand<'tcx>>],
    ) -> bool {
        let self_ref = extract_nth_argument_as_place(args, 0).unwrap_or_else(|| {
            panic!("BUG: `{function_name}` should receive a reference as a place")
        });
        let function = self.call_stack.peek();
        check_if_mutex_variable(&self_ref, function.def_id, self.tcx)
    }

    /// Call to a MIR function. It is the default for user-defined functions in the code.
    /// It is a recursive call for the translation process.
    ///
    /// A separate counter is incremented every time that
    /// the function is called to generate a unique label.
    fn call_mir_function(
        &mut self,
        function_def_id: rustc_hir::def_id::DefId,
        function_name: &str,
        args: &[rustc_span::source_map::Spanned<rustc_middle::mir::Operand<'tcx>>],
        places: Places,
    ) -> Option<Value> {
        let index = self.function_counter.get_count(function_name);

        match places {
            Places::WithCleanup {
                start_place,
                end_place,
                cleanup_place,
            } => {
                connect_places(
                    &mut self.net,
                    &start_place,
                    &cleanup_place,
                    &indexed_mir_function_cleanup_label(function_name, index),
                );

                let mir_function = MirFunction::new_with_mapped_args(
                    function_def_id,
                    indexed_mir_function_name(function_name, index),
                    start_place,
                    end_place,
                    args,
                    &self.call_stack.peek().memory,
                );
                self.call_stack.push(mir_function);
            }
            Places::Basic {
                start_place,
                end_place,
            } => {
                let mir_function = MirFunction::new_with_mapped_args(
                    function_def_id,
                    indexed_mir_function_name(function_name, index),
                    start_place,
                    end_place,
                    args,
                    &self.call_stack.peek().memory,
                );
                self.call_stack.push(mir_function);
            }
        }
        info!("Pushed function {function_name} to the translation call stack");
        self.translate_top_call_stack()
    }

    /// Call to `std::mem::drop`.
    /// Non-recursive call for the translation process.
    /// Returns `None` as there is no return value from this function.
    ///
    /// A separate counter is incremented every time that
    /// the function is called to generate a unique label.
    fn call_mem_drop(
        &mut self,
        function_name: &str,
        args: &[rustc_span::source_map::Spanned<rustc_middle::mir::Operand<'tcx>>],
        places: Places,
    ) -> Option<Value> {
        let index = self.function_counter.get_count(function_name);
        let transitions = call_foreign_function(function_name, index, places, &mut self.net);

        let dropped_place = extract_nth_argument_as_place(args, 0).unwrap_or_else(|| {
            panic!("BUG: `{function_name}` should receive the value to be dropped as a place")
        });

        let function = self.call_stack.peek_mut();
        let memory = &mut function.memory;
        let net = &mut self.net;
        match transitions {
            Transitions::Basic { default } => {
                mutex::handle_mutex_guard_drop(dropped_place, &default, net, memory);
            }
            Transitions::WithCleanup { default, cleanup } => {
                mutex::handle_mutex_guard_drop(dropped_place, &default, net, memory);
                mutex::handle_mutex_guard_drop(dropped_place, &cleanup, net, memory);
            }
        }
        None
    }

    /// Call to `std::ops::Deref::deref` or `std::ops::DerefMut::deref_mut`.
    /// Non-recursive call for the translation process.
    ///
    /// If a mutex guard is dereferenced mutably, add the transition to set the mutex condition later.
    ///
    /// In some cases, the `std::ops::Deref::deref` or `std::ops::DerefMut::deref_mut` functions contain a cleanup target.
    /// This target is not called in practice but creates trouble for lost signal detection.
    /// The reason is that any call may fail, which is equivalent to saying that the dereference operation
    /// was never present in the program, leading to a false lost signal.
    /// In conclusion: Ignore the cleanup place, do not model it.
    /// Assume `deref` and `deref_mut` never unwind when dereferencing a variable linked to a mutex or a mutex guard.
    ///
    /// A separate counter is incremented every time that
    /// the function is called to generate a unique label.
    fn call_deref_mutex(
        &mut self,
        function_name: &str,
        args: &[rustc_span::source_map::Spanned<rustc_middle::mir::Operand<'tcx>>],
        places: Places,
    ) -> Option<Value> {
        let index = self.function_counter.get_count(function_name);
        let places = places.ignore_cleanup_place();
        let transitions = call_foreign_function(function_name, index, places, &mut self.net);
        let transition = transitions.default();

        let Some(first_argument) = extract_nth_argument_as_place(args, 0) else {
            return None; // Nothing to return: Either the first argument is not present or it is a constant.
        };
        let current_function = self.call_stack.peek_mut();

        if function_name == "std::ops::DerefMut::deref_mut" {
            let mutex_guard_ref = current_function.memory.get_mutex_guard(&first_argument);
            mutex_guard_ref.mutex.add_deref_mut_transition(transition);
            info!("Encountered a mutable dereference of a mutex guard");
        }

        current_function
            .memory
            .get_linked_value_or_none(&first_argument)
    }

    /// Call to `std::result::Result::<T, E>::unwrap`.
    /// Non-recursive call for the translation process.
    ///
    /// In some cases, the `std::result::Result::<T, E>::unwrap` contain a cleanup target.
    /// This target is not called in practice but creates trouble for lost signal detection.
    /// The reason is that any call may fail, which is equivalent to saying that the lock operation
    /// was never present in the program, leading to a false lost signal.
    /// In conclusion: Ignore the cleanup place, do not model it.
    /// Assume `unwrap` never unwinds when applied to a variable linked to a mutex or a mutex guard.
    ///
    /// A separate counter is incremented every time that
    /// the function is called to generate a unique label.
    fn call_unwrap_mutex(
        &mut self,
        function_name: &str,
        args: &[rustc_span::source_map::Spanned<rustc_middle::mir::Operand<'tcx>>],
        places: Places,
    ) -> Option<Value> {
        let places = places.ignore_cleanup_place();
        let index = self.function_counter.get_count(function_name);
        call_foreign_function(function_name, index, places, &mut self.net);
        self.get_linked_value_in_first_argument(args)
    }

    /// Call to `std::thread::spawn`.
    /// Non-recursive call for the translation process.
    ///
    /// - Extracts the function `DefId` of the called function.
    /// - Extracts the closure for the thread.
    /// - Gets the sync variables passed in to the closure.
    /// - Adds the thread to the `ThreadManager`.
    /// - Links the return place to the `ThreadRef`.
    ///
    /// A separate counter is incremented every time that
    /// the function is called to generate a unique label.
    fn call_thread_spawn(
        &mut self,
        function_name: &str,
        args: &[rustc_span::source_map::Spanned<rustc_middle::mir::Operand<'tcx>>],
        destination: rustc_middle::mir::Place<'tcx>,
        places: Places,
    ) -> Value {
        let index = self.function_counter.get_count(function_name);
        let transitions = call_foreign_function(function_name, index, places, &mut self.net);
        let transition = transitions.default();

        // Extract the definition ID of the thread function
        let current_function = self.call_stack.peek_mut();
        let function_to_be_run = args.first().unwrap_or_else(|| {
            panic!("BUG: `{function_name}` should receive the function to be run")
        });
        let thread_function_def_id = extract_def_id_of_called_function_from_operand(
            &function_to_be_run.node,
            current_function.def_id,
            self.tcx,
        );

        // Create a new thread
        let index = self.threads.len();
        let (start_place, end_place) = Thread::create_start_and_end_places(&mut self.net, index);
        let closure_name = self.tcx.def_path_str(thread_function_def_id);

        let mir_function = MirFunction::new_with_mapped_args(
            thread_function_def_id,
            closure_name,
            start_place,
            end_place,
            args,
            &current_function.memory,
        );
        let thread = sync::thread::Thread::new(transition, mir_function, index);

        // The return value contains a new join handle. Link the local variable to it.
        let thread_ref = current_function
            .memory
            .link_join_handle(destination, thread);
        debug!("NEW JOIN HANDLE: {destination:?}");

        // Add the thread to the translator
        self.threads.push_back(thread_ref.clone());
        info!("Found thread {index} and pushed it to the back of the thread translation queue");
        Value::Single(Single::JoinHandle(thread_ref.clone()))
    }
}
