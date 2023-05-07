//! Submodule for defining the translator handler methods for every function call supported.

use super::Translator;
use crate::data_structures::petri_net_interface::{PlaceRef, TransitionRef};
use crate::naming::function::foreign_call_transition_labels;
use crate::translator::special_function::{call_foreign_function, is_foreign_function};
use crate::translator::sync::link_return_value_if_sync_variable;
use log::info;

/// A convenient typedef to pass the start place, the end place
/// and the (optional) cleanup place for a function call.
pub type FunctionPlaces = (PlaceRef, PlaceRef, Option<PlaceRef>);

impl<'tcx> Translator<'tcx> {
    /// Starts the corresponding handler for the function call.
    /// Checks if the function is one of the
    /// supported synchronization or multithreading functions,
    /// then if the function is a foreign function call and
    /// lastly handle the standard MIR function case.
    pub fn start_function_call(
        &mut self,
        function_def_id: rustc_hir::def_id::DefId,
        args: &[rustc_middle::mir::Operand<'tcx>],
        destination: rustc_middle::mir::Place<'tcx>,
        function_call_places: FunctionPlaces,
    ) {
        let function_name = self.tcx.def_path_str(function_def_id);
        match function_name.as_str() {
            "std::sync::Condvar::new" => {
                self.call_condvar_new(&function_name, args, destination, &function_call_places);
                return;
            }
            "std::sync::Condvar::notify_one" => {
                self.call_condvar_notify_one(
                    &function_name,
                    args,
                    destination,
                    &function_call_places,
                );
                return;
            }
            "std::sync::Condvar::wait" => {
                self.call_condvar_wait(args, destination, &function_call_places);
                return;
            }
            "std::sync::Mutex::<T>::lock" => {
                self.call_mutex_lock(&function_name, args, destination, &function_call_places);
                return;
            }
            "std::sync::Mutex::<T>::new" => {
                self.call_mutex_new(&function_name, args, destination, &function_call_places);
                return;
            }
            "std::thread::spawn" => {
                self.call_thread_spawn(&function_name, args, destination, &function_call_places);
                return;
            }
            "std::thread::JoinHandle::<T>::join" => {
                self.call_thread_join(&function_name, args, destination, &function_call_places);
                return;
            }
            _ => {}
        }
        // Default case for standard and core library calls
        if is_foreign_function(function_def_id, self.tcx) {
            self.call_foreign_function(&function_name, args, destination, &function_call_places);
            return;
        }
        // Default case: A function with MIR representation
        let (start_place, end_place, _) = function_call_places;
        self.push_function_to_call_stack(function_def_id, start_place, end_place);
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
    /// Returns the transition that represents the function call.
    fn call_foreign_function(
        &mut self,
        function_name: &str,
        args: &[rustc_middle::mir::Operand<'tcx>],
        destination: rustc_middle::mir::Place<'tcx>,
        function_call_places: &FunctionPlaces,
    ) -> TransitionRef {
        let index = self.function_counter.get_count(function_name);
        self.function_counter.increment(function_name);
        let function_transition = call_foreign_function(
            function_call_places,
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

        function_transition
    }

    /// Call to `std::sync::Condvar::new`.
    /// Non-recursive call for the translation process.
    fn call_condvar_new(
        &mut self,
        function_name: &str,
        args: &[rustc_middle::mir::Operand<'tcx>],
        destination: rustc_middle::mir::Place<'tcx>,
        function_call_places: &FunctionPlaces,
    ) {
        self.call_foreign_function(function_name, args, destination, function_call_places);

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
        function_call_places: &FunctionPlaces,
    ) {
        let notify_one_transition =
            self.call_foreign_function(function_name, args, destination, function_call_places);

        let current_function = self.call_stack.peek_mut();
        self.condvar_manager.translate_side_effects_notify_one(
            args,
            &notify_one_transition,
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
        function_call_places: &FunctionPlaces,
    ) {
        let wait_transitions = self
            .condvar_manager
            .translate_call_wait(function_call_places, &mut self.net);

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
        function_call_places: &FunctionPlaces,
    ) {
        let transition_function_call =
            self.call_foreign_function(function_name, args, destination, function_call_places);

        let current_function = self.call_stack.peek_mut();
        self.mutex_manager.translate_side_effects_lock(
            args,
            destination,
            &transition_function_call,
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
        function_call_places: &FunctionPlaces,
    ) {
        self.call_foreign_function(function_name, args, destination, function_call_places);

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
        function_call_places: &FunctionPlaces,
    ) {
        let transition_function_call =
            self.call_foreign_function(function_name, args, destination, function_call_places);

        let current_function = self.call_stack.peek();
        self.thread_manager.translate_side_effects_join(
            args,
            transition_function_call,
            &current_function.memory,
        );
    }

    /// Call to `std::thread::spawn`.
    /// Non-recursive call for the translation process.
    fn call_thread_spawn(
        &mut self,
        function_name: &str,
        args: &[rustc_middle::mir::Operand<'tcx>],
        destination: rustc_middle::mir::Place<'tcx>,
        function_call_places: &FunctionPlaces,
    ) {
        let transition_function_call =
            self.call_foreign_function(function_name, args, destination, function_call_places);

        let current_function = self.call_stack.peek_mut();
        self.thread_manager.translate_side_effects_spawn(
            args,
            destination,
            transition_function_call,
            &mut current_function.memory,
            current_function.def_id,
            self.tcx,
        );
    }
}
