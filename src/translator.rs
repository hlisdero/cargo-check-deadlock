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

use crate::data_structures::hash_map_counter::HashMapCounter;
use crate::data_structures::petri_net_interface::{connect_places, PetriNet, PlaceRef};
use crate::data_structures::stack::Stack;
use crate::naming::function::{
    foreign_call_transition_labels, indexed_mir_function_cleanup_label, indexed_mir_function_name,
};
use crate::naming::program::{PROGRAM_END, PROGRAM_PANIC, PROGRAM_START};
use crate::translator::function::{Places, Transitions};
use crate::translator::mir_function::MirFunction;
use crate::translator::special_function::{call_foreign_function, is_foreign_function};
use crate::utils::{
    extract_closure, extract_def_id_of_called_function_from_operand, extract_nth_argument_as_place,
};
use log::{debug, info};
use rustc_middle::mir::visit::Visitor;
use rustc_middle::mir::UnwindAction;
use special_function::{call_diverging_function, call_panic_function, is_panic_function};
use std::cell::RefCell;
use std::rc::Rc;
use sync::mutex;
use sync::thread::Thread;

pub struct Translator<'tcx> {
    tcx: rustc_middle::ty::TyCtxt<'tcx>,
    net: PetriNet,
    program_start: PlaceRef,
    program_end: PlaceRef,
    program_panic: PlaceRef,
    call_stack: Stack<MirFunction<'tcx>>,
    function_counter: HashMapCounter,
    threads: Vec<std::rc::Rc<std::cell::RefCell<Thread>>>,
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
            threads: Vec::new(),
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

    /// Main translation loop for the threads.
    /// Iterate over the threads found and translate them.
    /// If sync variables were passed to the thread, move them to the memory of the thread function.
    /// Replaces the program panic place with the thread's end place
    /// since abnormal thread termination does not affect the main thread.
    fn translate_threads(&mut self) {
        for thread in std::mem::take(&mut self.threads) {
            let mut thread = thread.borrow_mut();
            let index = thread.index;

            info!("Starting translating thread {}", index);
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
            info!("Finished translating thread {}", index);
        }
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
        unwind: UnwindAction,
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
            (Some(return_block), UnwindAction::Terminate) => {
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
            (None, UnwindAction::Terminate) => {
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
                return;
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

        self.start_function_call(function_def_id, &function_name, args, destination, places);
        self.function_counter.increment(&function_name);
    }

    /// Starts the corresponding handler for the function call.
    /// Checks if the function is one of the
    /// supported synchronization or multithreading functions,
    /// then if the function is a foreign function call and
    /// lastly handle the standard MIR function case.
    pub fn start_function_call(
        &mut self,
        function_def_id: rustc_hir::def_id::DefId,
        function_name: &str,
        args: &[rustc_middle::mir::Operand<'tcx>],
        destination: rustc_middle::mir::Place<'tcx>,
        places: Places,
    ) {
        // Special cases
        if function_name == "std::mem::drop" {
            self.call_mem_drop(function_name, args, destination, places);
            return;
        }
        if function_name == "std::result::Result::<T, E>::unwrap" {
            self.call_result_unwrap(function_name, args, destination, places);
            return;
        }
        if function_name == "std::thread::spawn" {
            self.call_thread_spawn(function_name, args, destination, places);
            return;
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

            sync::call_function(function_name, index, args, destination, places, net, memory);
            return;
        }
        // Default case for standard and core library calls
        if is_foreign_function(function_def_id, function_name, self.tcx) {
            self.call_foreign_function(function_name, args, destination, places);
            return;
        }
        // Default case: A function with MIR representation
        self.call_mir_function(function_def_id, function_name, places);
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
        places: Places,
    ) {
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

                self.call_stack.push(MirFunction::new(
                    function_def_id,
                    indexed_mir_function_name(function_name, index),
                    start_place,
                    end_place,
                ));
            }
            Places::Basic {
                start_place,
                end_place,
            } => {
                self.call_stack.push(MirFunction::new(
                    function_def_id,
                    indexed_mir_function_name(function_name, index),
                    start_place,
                    end_place,
                ));
            }
        }
        info!("Pushed function {function_name} to the translation call stack");
        self.translate_top_call_stack();
    }

    /// Call to a foreign function. It is the default for standard and core library calls.
    /// It is a non-recursive call for the translation process.
    /// It is also reused by other handlers since this is the basic case.
    ///
    /// Translates a call to a function with given function name using
    /// the same representation as in `special_function::call_foreign_function`.
    ///
    /// A separate counter is incremented every time that
    /// the function is called to generate a unique label.
    ///
    /// Performs a check to keep track of synchronization primitives.
    /// In case the first argument is a mutex, mutex guard, join handle or condition variable,
    /// it links the first argument of the function to its return value.
    ///
    /// Returns the transitions representing the function call.
    fn call_foreign_function(
        &mut self,
        function_name: &str,
        args: &[rustc_middle::mir::Operand<'tcx>],
        destination: rustc_middle::mir::Place<'tcx>,
        places: Places,
    ) -> Transitions {
        let index = self.function_counter.get_count(function_name);
        let transitions = call_foreign_function(
            places,
            &foreign_call_transition_labels(function_name, index),
            &mut self.net,
        );

        let current_function = self.call_stack.peek_mut();
        sync::link_return_value_if_sync_variable(
            args,
            destination,
            &mut current_function.memory,
            current_function.def_id,
            self.tcx,
        );

        transitions
    }

    /// Call to `std::mem::drop`.
    /// Non-recursive call for the translation process.
    fn call_mem_drop(
        &mut self,
        function_name: &str,
        args: &[rustc_middle::mir::Operand<'tcx>],
        destination: rustc_middle::mir::Place<'tcx>,
        places: Places,
    ) {
        let transitions = self.call_foreign_function(function_name, args, destination, places);

        let Some(dropped_place) = extract_nth_argument_as_place(args, 0) else {
            panic!("BUG: `std::mem::drop` should receive the value to be dropped as a place");
        };

        let function = self.call_stack.peek_mut();
        let memory = &mut function.memory;
        let net = &mut self.net;
        match transitions {
            Transitions::Basic { transition } => {
                mutex::handle_mutex_guard_drop(dropped_place, &transition, net, memory);
            }
            Transitions::WithCleanup {
                transition,
                cleanup_transition,
            } => {
                mutex::handle_mutex_guard_drop(dropped_place, &transition, net, memory);
                mutex::handle_mutex_guard_drop(dropped_place, &cleanup_transition, net, memory);
            }
        }
    }

    /// Call to `std::result::Result::<T, E>::unwrap`.
    /// Non-recursive call for the translation process.
    ///
    /// There is an important detail regarding how rustc interprets this call in conjunction with mutexes.
    /// In some cases, the `std::result::Result::<T, E>::unwrap` function applied
    /// to the return value of `std::sync::Mutex::<T>::lock` does not generate code to drop the mutex guard
    /// since it considers that the mutex was never locked in the first place.
    /// Nonetheless, for the purposes of the Petri net model, it is necessary to unlock the mutex in this case.
    fn call_result_unwrap(
        &mut self,
        function_name: &str,
        args: &[rustc_middle::mir::Operand<'tcx>],
        destination: rustc_middle::mir::Place<'tcx>,
        places: Places,
    ) {
        let transitions = self.call_foreign_function(function_name, args, destination, places);

        if let Transitions::WithCleanup {
            cleanup_transition, ..
        } = transitions
        {
            let unwrapped_place = extract_nth_argument_as_place(args, 0)
            .expect("BUG: `std::result::Result::<T, E>::unwrap` should receive the value to be unwrapped as a place");
            let function = self.call_stack.peek_mut();
            let memory = &mut function.memory;
            let net = &mut self.net;
            mutex::handle_mutex_guard_drop(unwrapped_place, &cleanup_transition, net, memory);
        }
    }

    /// Call to `std::thread::spawn`.
    /// Non-recursive call for the translation process.
    ///
    /// - Extracts the function `DefId` of the called function.
    /// - Extracts the closure for the thread.
    /// - Finds the sync variables passed in to the closure.
    /// - Adds the thread to the `ThreadManager`.
    /// - Links the return place to the `ThreadRef`.
    fn call_thread_spawn(
        &mut self,
        function_name: &str,
        args: &[rustc_middle::mir::Operand<'tcx>],
        destination: rustc_middle::mir::Place<'tcx>,
        places: Places,
    ) {
        let transitions = self.call_foreign_function(function_name, args, destination, places);
        let transition = match transitions {
            Transitions::Basic { transition } | Transitions::WithCleanup { transition, .. } => {
                transition
            }
        };
        // Extract the definition ID of the thread function
        let current_function = self.call_stack.peek_mut();
        let function_to_be_run = args
            .get(0)
            .expect("BUG: `std::thread::spawn` should receive the function to be run");
        let thread_function_def_id = extract_def_id_of_called_function_from_operand(
            function_to_be_run,
            current_function.def_id,
            self.tcx,
        );

        // Extract the closure and find the sync variables passed in to the closure
        let closure_for_spawn = extract_closure(args);
        let memory_entries =
            sync::thread::find_sync_variables(closure_for_spawn, &mut current_function.memory);

        // Add a new thread
        let index = self.threads.len();
        let thread = Rc::new(RefCell::new(sync::thread::Thread::new(
            transition,
            thread_function_def_id,
            memory_entries.0,
            memory_entries.1,
            memory_entries.2,
            memory_entries.3,
            index,
        )));
        let thread_ref = Rc::clone(&thread);
        self.threads.push(thread);
        info!("Found thread {index} and pushed it to the back of the thread translation queue");

        // The return value contains a new join handle. Link the local variable to it.
        current_function
            .memory
            .link_place_to_join_handle(destination, &thread_ref);
        debug!("NEW JOIN HANDLE: {destination:?}");
    }
}
