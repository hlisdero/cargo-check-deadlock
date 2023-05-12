//! Submodule for defining the translator handler methods for every function call supported.

use super::Translator;

use crate::naming::function::{foreign_call_transition_labels, indexed_mir_function_name};
use crate::translator::function::{Places, Transitions};
use crate::translator::mir_function::MirFunction;
use crate::translator::special_function::{call_foreign_function, is_foreign_function};
use crate::translator::sync::link_return_value_if_sync_variable;
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
        if self.check_supported_sync_function(function_name, args, destination, &places) {
            return;
        }
        // Default case for standard and core library calls
        if is_foreign_function(function_def_id, function_name, self.tcx) {
            self.call_foreign_function(function_name, args, destination, &places);
            return;
        }
        // Default case: A function with MIR representation
        self.call_mir_function(function_def_id, function_name, places);
    }

    /// Checks if the function name corresponds to one of the
    /// supported synchronization or multithreading functions.
    /// If this is the case, calls the corresponding handler and returns `true`.
    /// Otherwise, it returns `false`.
    fn check_supported_sync_function(
        &mut self,
        function_name: &str,
        args: &[rustc_middle::mir::Operand<'tcx>],
        destination: rustc_middle::mir::Place<'tcx>,
        places: &Places,
    ) -> bool {
        match function_name {
            "std::mem::drop" => {
                self.call_mem_drop(function_name, args, destination, places);
                true
            }
            "std::sync::Condvar::new" => {
                self.call_condvar_new(function_name, args, destination, places);
                true
            }
            "std::sync::Condvar::notify_one" => {
                self.call_condvar_notify_one(function_name, args, destination, places);
                true
            }
            "std::sync::Condvar::wait" => {
                self.call_condvar_wait(args, destination, places);
                true
            }
            "std::sync::Mutex::<T>::lock" => {
                self.call_mutex_lock(function_name, args, destination, places);
                true
            }
            "std::sync::Mutex::<T>::new" => {
                self.call_mutex_new(function_name, args, destination, places);
                true
            }
            "std::thread::spawn" => {
                self.call_thread_spawn(function_name, args, destination, places);
                true
            }
            "std::thread::JoinHandle::<T>::join" => {
                self.call_thread_join(function_name, args, destination, places);
                true
            }
            _ => false,
        }
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

        let (start_place, end_place) = match places {
            Places::WithCleanup { .. } => {
                panic!("BUG: Function with MIR representation should not have a cleanup place");
            }
            Places::Basic {
                start_place,
                end_place,
            } => (start_place, end_place),
        };
        self.call_stack.push(MirFunction::new(
            function_def_id,
            indexed_mir_function_name(function_name, index),
            start_place,
            end_place,
        ));
        info!("Pushed function {function_name} to the translation callstack");
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
    /// In case the first argument is a mutex, lock guard, join handle or condition variable,
    /// it links the first argument of the function to its return value.
    ///
    /// Returns the transitions representing the function call.
    fn call_foreign_function(
        &mut self,
        function_name: &str,
        args: &[rustc_middle::mir::Operand<'tcx>],
        destination: rustc_middle::mir::Place<'tcx>,
        places: &Places,
    ) -> Transitions {
        let index = self.function_counter.get_count(function_name);
        self.function_counter.increment(function_name);
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
        places: &Places,
    ) {
        let transitions = self.call_foreign_function(function_name, args, destination, places);

        let current_function = self.call_stack.peek_mut();
        let Some(dropped_place) = extract_nth_argument_as_place(args, 0) else {
            panic!("BUG: `std::mem::drop` should receive the value to be dropped as a place");
        };
        self.mutex_manager.handle_lock_guard_drop(
            dropped_place,
            transitions.get_transition(),
            &current_function.memory,
            &mut self.net,
        );
    }

    /// Call to `std::sync::Condvar::new`.
    /// Non-recursive call for the translation process.
    fn call_condvar_new(
        &mut self,
        function_name: &str,
        args: &[rustc_middle::mir::Operand<'tcx>],
        destination: rustc_middle::mir::Place<'tcx>,
        places: &Places,
    ) {
        self.call_foreign_function(function_name, args, destination, places);

        let current_function = self.call_stack.peek_mut();
        self.condvar_manager.translate_side_effects_new(
            destination,
            &mut self.net,
            &mut current_function.memory,
        );
    }

    /// Call to `std::sync::Condvar::notify_one`.
    /// Non-recursive call for the translation process.
    fn call_condvar_notify_one(
        &mut self,
        function_name: &str,
        args: &[rustc_middle::mir::Operand<'tcx>],
        destination: rustc_middle::mir::Place<'tcx>,
        places: &Places,
    ) {
        let transitions = self.call_foreign_function(function_name, args, destination, places);

        let current_function = self.call_stack.peek_mut();
        self.condvar_manager.translate_side_effects_notify_one(
            args,
            transitions.get_transition(),
            &mut self.net,
            &mut current_function.memory,
        );
    }

    /// Call to `std::sync::Condvar::wait`.
    /// Non-recursive call for the translation process.
    fn call_condvar_wait(
        &mut self,
        args: &[rustc_middle::mir::Operand<'tcx>],
        destination: rustc_middle::mir::Place<'tcx>,
        places: &Places,
    ) {
        let wait_transitions = self
            .condvar_manager
            .translate_call_wait(places, &mut self.net);

        let current_function = self.call_stack.peek_mut();
        self.condvar_manager.translate_side_effects_wait(
            args,
            destination,
            &wait_transitions,
            &mut self.net,
            &mut self.mutex_manager,
            &mut current_function.memory,
        );
    }

    /// Call to `std::sync::Mutex::<T>::lock`.
    /// Non-recursive call for the translation process.
    fn call_mutex_lock(
        &mut self,
        function_name: &str,
        args: &[rustc_middle::mir::Operand<'tcx>],
        destination: rustc_middle::mir::Place<'tcx>,
        places: &Places,
    ) {
        let transitions = self.call_foreign_function(function_name, args, destination, places);

        let current_function = self.call_stack.peek_mut();
        self.mutex_manager.translate_side_effects_lock(
            args,
            destination,
            transitions,
            &mut self.net,
            &mut current_function.memory,
        );
    }

    /// Call to `std::sync::Mutex::<T>::new`.
    /// Non-recursive call for the translation process.
    fn call_mutex_new(
        &mut self,
        function_name: &str,
        args: &[rustc_middle::mir::Operand<'tcx>],
        destination: rustc_middle::mir::Place<'tcx>,
        places: &Places,
    ) {
        self.call_foreign_function(function_name, args, destination, places);

        let current_function = self.call_stack.peek_mut();
        self.mutex_manager.translate_side_effects_new(
            destination,
            &mut self.net,
            &mut current_function.memory,
        );
    }

    /// Call to `std::thread::JoinHandle::<T>::join`.
    /// Non-recursive call for the translation process.
    fn call_thread_join(
        &mut self,
        function_name: &str,
        args: &[rustc_middle::mir::Operand<'tcx>],
        destination: rustc_middle::mir::Place<'tcx>,
        places: &Places,
    ) {
        let transitions = self.call_foreign_function(function_name, args, destination, places);
        let transition = match transitions {
            Transitions::Basic { transition } | Transitions::WithCleanup { transition, .. } => {
                transition
            }
        };

        let current_function = self.call_stack.peek();
        self.thread_manager
            .translate_side_effects_join(args, transition, &current_function.memory);
    }

    /// Call to `std::thread::spawn`.
    /// Non-recursive call for the translation process.
    fn call_thread_spawn(
        &mut self,
        function_name: &str,
        args: &[rustc_middle::mir::Operand<'tcx>],
        destination: rustc_middle::mir::Place<'tcx>,
        places: &Places,
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
