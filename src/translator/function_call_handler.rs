//! Submodule for defining the translator handler methods for every function call supported.

use super::Translator;

use crate::data_structures::petri_net_interface::connect_places;
use crate::naming::function::{
    foreign_call_transition_labels, indexed_mir_function_cleanup_label, indexed_mir_function_name,
};
use crate::translator::function::{Places, Transitions};
use crate::translator::mir_function::MirFunction;
use crate::translator::special_function::{call_foreign_function, is_foreign_function};
use crate::translator::sync::condvar;
use crate::translator::sync::mutex;
use crate::translator::sync::{is_supported_function, link_return_value_if_sync_variable};
use crate::translator::ThreadManager;
use crate::utils::extract_nth_argument_as_place;
use log::info;

impl<'tcx> Translator<'tcx> {
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
        // Sync or multithreading function
        if is_supported_function(function_name) {
            self.call_supported_sync_function(function_name, args, destination, places);
            return;
        }
        // Default case for standard and core library calls
        if is_foreign_function(function_def_id, function_name, self.tcx) {
            self.call_foreign_function(function_name, args, destination, places);
            self.function_counter.increment(function_name);
            return;
        }
        // Default case: A function with MIR representation
        self.call_mir_function(function_def_id, function_name, places);
    }

    /// Calls the corresponding handler for the supported synchronization or multithreading functions.
    fn call_supported_sync_function(
        &mut self,
        function_name: &str,
        args: &[rustc_middle::mir::Operand<'tcx>],
        destination: rustc_middle::mir::Place<'tcx>,
        places: Places,
    ) {
        // Index for transition and place labels
        let index = self.function_counter.get_count(function_name);
        // A reference to the memory of the current function
        let current_function = self.call_stack.peek_mut();
        let memory = &mut current_function.memory;
        // A reference to the Petri net to add transitions and places
        let net = &mut self.net;

        match function_name {
            "std::mem::drop" => {
                self.call_mem_drop(function_name, args, destination, places);
            }
            "std::result::Result::<T, E>::unwrap" => {
                self.call_result_unwrap(function_name, args, destination, places);
            }
            "std::sync::Condvar::new" => {
                condvar::call_new(function_name, index, destination, places, net, memory);
            }
            "std::sync::Condvar::notify_one" => {
                condvar::call_notify_one(function_name, index, args, places, net, memory);
            }
            "std::sync::Condvar::wait" => {
                condvar::call_wait(index, args, destination, places, net, memory);
            }
            "std::sync::Mutex::<T>::lock" => {
                mutex::call_lock(function_name, index, args, destination, places, net, memory);
            }
            "std::sync::Mutex::<T>::new" => {
                mutex::call_new(function_name, index, destination, places, net, memory);
            }
            "std::thread::spawn" => {
                self.call_thread_spawn(function_name, args, destination, places);
            }
            "std::thread::JoinHandle::<T>::join" => {
                self.call_thread_join(function_name, args, destination, places);
            }
            _ => panic!("BUG: Call handler for {function_name} is not defined"),
        }
        // Increment the counter at the end of every call to prepare the new label.
        self.function_counter.increment(function_name);
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
        self.function_counter.increment(function_name);

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
        link_return_value_if_sync_variable(
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
        match transitions {
            Transitions::Basic { transition } => {
                self.handle_mutex_guard_drop(dropped_place, &transition);
            }
            Transitions::WithCleanup {
                transition,
                cleanup_transition,
            } => {
                self.handle_mutex_guard_drop(dropped_place, &transition);
                self.handle_mutex_guard_drop(dropped_place, &cleanup_transition);
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
            self.handle_mutex_guard_drop(unwrapped_place, &cleanup_transition);
        }
    }

    /// Call to `std::thread::JoinHandle::<T>::join`.
    /// Non-recursive call for the translation process.
    fn call_thread_join(
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

        let current_function = self.call_stack.peek();
        ThreadManager::translate_side_effects_join(args, transition, &current_function.memory);
    }

    /// Call to `std::thread::spawn`.
    /// Non-recursive call for the translation process.
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

        let current_function = self.call_stack.peek_mut();
        self.thread_manager.translate_side_effects_spawn(
            args,
            destination,
            transition,
            &mut current_function.memory,
            current_function.def_id,
            self.tcx,
        );
    }
}
