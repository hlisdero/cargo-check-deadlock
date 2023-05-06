//! Submodule for defining the translator handler methods for every variant of `FunctionCall`.

use super::Translator;
use crate::data_structures::petri_net_interface::TransitionRef;
use crate::naming::function::foreign_call_transition_labels;
use crate::translator::function_call::{FunctionCall, FunctionPlaces};
use crate::translator::special_function::call_foreign_function;
use crate::translator::sync::link_return_value_if_sync_variable;
use log::info;

impl<'tcx> Translator<'tcx> {
    /// Processes the `FunctionCall` enum and
    /// calls the corresponding handler for the function call type.
    pub fn start_function_call(
        &mut self,
        function_call: &FunctionCall,
        function_def_id: rustc_hir::def_id::DefId,
        args: &[rustc_middle::mir::Operand<'tcx>],
        destination: rustc_middle::mir::Place<'tcx>,
        function_call_places: FunctionPlaces,
    ) {
        let function_name = self.tcx.def_path_str(function_def_id);

        match function_call {
            FunctionCall::ForeignWithSyncPrimitive => {
                self.call_foreign_function_with_sync_primitive(
                    &function_name,
                    args,
                    destination,
                    &function_call_places,
                );
            }
            FunctionCall::CondVarNew => {
                self.call_condvar_new(&function_name, destination, &function_call_places);
            }
            FunctionCall::CondVarNotifyOne => {
                self.call_condvar_notify_one(&function_name, args, &function_call_places);
            }
            FunctionCall::CondVarWait => {
                self.call_condvar_wait(args, destination, &function_call_places);
            }
            FunctionCall::Foreign => {
                self.call_foreign_function(&function_name, &function_call_places);
            }
            FunctionCall::MirFunction => {
                let (start_place, end_place, _) = function_call_places;
                self.push_function_to_call_stack(function_def_id, start_place, end_place);
                info!("Pushed function {function_name} to the translation callstack");
                self.translate_top_call_stack();
            }
            FunctionCall::MutexLock => {
                self.call_mutex_lock(&function_name, args, destination, &function_call_places);
            }
            FunctionCall::MutexNew => {
                self.call_mutex_new(&function_name, destination, &function_call_places);
            }
            FunctionCall::ThreadJoin => {
                self.call_thread_join(&function_name, args, &function_call_places);
            }
            FunctionCall::ThreadSpawn => {
                self.call_thread_spawn(&function_name, args, destination, &function_call_places);
            }
        }
    }

    /// Handler for the case `FunctionCall::Foreign`.
    /// It is also reused by other handlers since this is the basic case.
    ///
    /// Translates a call to a function with given function name using
    /// the same representation as in `special_function::call_foreign_function`.
    ///
    /// A separate counter is incremented every time that
    /// the function is called to generate a unique label.
    ///
    /// Returns the transition that represents the function call.
    fn call_foreign_function(
        &mut self,
        function_name: &str,
        function_call_places: &FunctionPlaces,
    ) -> TransitionRef {
        let index = self.function_counter.get_count(function_name);
        self.function_counter.increment(function_name);
        call_foreign_function(
            function_call_places,
            &foreign_call_transition_labels(function_name, index),
            &mut self.net,
        )
    }

    /// Handler for the case `FunctionCall::ForeignWithSyncPrimitive`.
    /// It is an extension of `call_foreign_function` that performs a check
    /// to keep track of synchronization primitives.
    /// The goal is to link the first argument of the function to its return value
    /// in case the first argument is a mutex, lock guard, join handle or condition variable.
    fn call_foreign_function_with_sync_primitive(
        &mut self,
        function_name: &str,
        args: &[rustc_middle::mir::Operand<'tcx>],
        destination: rustc_middle::mir::Place<'tcx>,
        function_call_places: &FunctionPlaces,
    ) {
        self.call_foreign_function(function_name, function_call_places);

        let current_function = self.call_stack.peek_mut();
        link_return_value_if_sync_variable(
            args,
            destination,
            &mut current_function.memory,
            current_function.def_id,
            self.tcx,
        );
    }

    /// Handler for the case `FunctionCall::CondvarNew`.
    fn call_condvar_new(
        &mut self,
        function_name: &str,
        destination: rustc_middle::mir::Place<'tcx>,
        function_call_places: &FunctionPlaces,
    ) {
        self.call_foreign_function(function_name, function_call_places);

        let current_function = self.call_stack.peek_mut();
        self.condvar_manager.translate_side_effects_new(
            destination,
            &mut self.net,
            &mut current_function.memory,
        );
    }

    /// Handler for the case `FunctionCall::CondVarNotifyOne`.
    fn call_condvar_notify_one(
        &mut self,
        function_name: &str,
        args: &[rustc_middle::mir::Operand<'tcx>],
        function_call_places: &FunctionPlaces,
    ) {
        let notify_one_transition = self.call_foreign_function(function_name, function_call_places);

        let current_function = self.call_stack.peek_mut();
        self.condvar_manager.translate_side_effects_notify_one(
            args,
            &notify_one_transition,
            &mut self.net,
            &mut current_function.memory,
        );
    }

    /// Handler for the case `FunctionCall::CondVarWait`.
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

    /// Handler for the case `FunctionCall::MutexLock`.
    fn call_mutex_lock(
        &mut self,
        function_name: &str,
        args: &[rustc_middle::mir::Operand<'tcx>],
        destination: rustc_middle::mir::Place<'tcx>,
        function_call_places: &FunctionPlaces,
    ) {
        let transition_function_call =
            self.call_foreign_function(function_name, function_call_places);

        let current_function = self.call_stack.peek_mut();
        self.mutex_manager.translate_side_effects_lock(
            args,
            destination,
            &transition_function_call,
            &mut self.net,
            &mut current_function.memory,
        );
    }

    /// Handler for the case `FunctionCall::MutexNew`.
    fn call_mutex_new(
        &mut self,
        function_name: &str,
        destination: rustc_middle::mir::Place<'tcx>,
        function_call_places: &FunctionPlaces,
    ) {
        self.call_foreign_function(function_name, function_call_places);

        let current_function = self.call_stack.peek_mut();
        self.mutex_manager.translate_side_effects_new(
            destination,
            &mut self.net,
            &mut current_function.memory,
        );
    }

    /// Handler for the case `FunctionCall::ThreadJoin`.
    fn call_thread_join(
        &mut self,
        function_name: &str,
        args: &[rustc_middle::mir::Operand<'tcx>],
        function_call_places: &FunctionPlaces,
    ) {
        let transition_function_call =
            self.call_foreign_function(function_name, function_call_places);

        let current_function = self.call_stack.peek();
        self.thread_manager.translate_side_effects_join(
            args,
            transition_function_call,
            &current_function.memory,
        );
    }

    /// Handler for the case `FunctionCall::ThreadSpawn`.
    fn call_thread_spawn(
        &mut self,
        function_name: &str,
        args: &[rustc_middle::mir::Operand<'tcx>],
        destination: rustc_middle::mir::Place<'tcx>,
        function_call_places: &FunctionPlaces,
    ) {
        let transition_function_call =
            self.call_foreign_function(function_name, function_call_places);

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
