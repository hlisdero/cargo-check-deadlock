//! Submodule for defining the function calls supported by the translator and
//! the specific handler methods for every one of them.

use super::Translator;
use crate::naming::function::foreign_call_transition_labels;
use crate::translator::special_function::{call_foreign_function, is_foreign_function};
use crate::translator::sync::ArcManager;
use crate::utils::extract_nth_argument;
use netcrab::petri_net::PlaceRef;

/// A convenient typedef to pass the start place, the end place
/// and the (optional) cleanup place for a function call.
pub type FunctionPlaces = (PlaceRef, PlaceRef, Option<PlaceRef>);

/// Types of function calls that the translator supports.
pub enum FunctionCall {
    /// Call to `std::sync::Arc::<T>::new`
    /// Non-recursive call for the translation process.
    ArcNew,
    /// Call to `std::clone::Clone::clone`
    /// Non-recursive call for the translation process.
    Clone,
    /// Call to `std::sync::Condvar::new`
    /// Non-recursive call for the translation process.
    CondVarNew,
    /// Call to `std::sync::Condvar::notify_one`
    /// Non-recursive call for the translation process.
    CondVarNotifyOne,
    /// Call to `std::sync::Condvar::wait`
    /// Non-recursive call for the translation process.
    CondVarWait,
    /// Call to `std::ops::Deref::deref`
    /// Non-recursive call for the translation process.
    Deref,
    /// Abridged function call.
    /// Non-recursive call for the translation process.
    Foreign,
    /// MIR function call (the "default" case).
    /// Recursive call for the translation process.
    MirFunction,
    /// Call to `std::sync::Mutex::<T>::lock`.
    /// Non-recursive call for the translation process.
    MutexLock,
    /// Call to `std::sync::Mutex::<T>::new`.
    /// Non-recursive call for the translation process.
    MutexNew,
    /// Call to `std::thread::JoinHandle::<T>::join`.
    /// Non-recursive call for the translation process.
    ThreadJoin,
    /// Call to `std::thread::spawn`.
    /// Non-recursive call for the translation process.
    ThreadSpawn,
    /// Call to `std::result::Result::<T, E>::unwrap`.
    /// Non-recursive call for the translation process.
    Unwrap,
}

impl FunctionCall {
    /// Creates a new function call depending on the specific function that will be called.
    pub fn new(function_def_id: rustc_hir::def_id::DefId, tcx: rustc_middle::ty::TyCtxt) -> Self {
        let function_name = tcx.def_path_str(function_def_id);

        if let Some(function_call) = Self::is_supported_function(&function_name) {
            return function_call;
        }
        // Default case for standard and core library calls
        if is_foreign_function(function_def_id, tcx) {
            return Self::Foreign;
        }
        // Default case: A function with MIR representation
        Self::MirFunction
    }

    /// Checks if the function is one of the supported synchronization or
    /// multithreading functions.
    /// Returns the corresponding variant for the function or `None` otherwise.
    fn is_supported_function(function_name: &str) -> Option<Self> {
        match function_name {
            "std::clone::Clone::clone" => Some(Self::Clone),
            "std::ops::Deref::deref" => Some(Self::Deref),
            "std::result::Result::<T, E>::unwrap" => Some(Self::Unwrap),
            "std::sync::Arc::<T>::new" => Some(Self::ArcNew),
            "std::sync::Condvar::new" => Some(Self::CondVarNew),
            "std::sync::Condvar::notify_one" => Some(Self::CondVarNotifyOne),
            "std::sync::Condvar::wait" => Some(Self::CondVarWait),
            "std::sync::Mutex::<T>::new" => Some(Self::MutexNew),
            "std::sync::Mutex::<T>::lock" => Some(Self::MutexLock),
            "std::thread::spawn" => Some(Self::ThreadSpawn),
            "std::thread::JoinHandle::<T>::join" => Some(Self::ThreadJoin),
            _ => None,
        }
    }
}

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
            FunctionCall::ArcNew => {
                self.call_arc_new(args, destination, &function_call_places);
            }
            FunctionCall::Clone => {
                self.call_clone(args, destination, &function_call_places);
            }
            FunctionCall::CondVarNew => {
                self.call_condvar_new(destination, &function_call_places);
            }
            FunctionCall::CondVarNotifyOne => {
                self.call_condvar_notify_one(args, &function_call_places);
            }
            FunctionCall::CondVarWait => {
                self.call_condvar_wait(args, &function_call_places);
            }
            FunctionCall::Deref => {
                self.call_deref(args, destination, &function_call_places);
            }
            FunctionCall::MirFunction => {
                self.call_mir_function(function_def_id, function_call_places);
            }
            FunctionCall::Foreign => {
                self.call_foreign(&function_name, &function_call_places);
            }
            FunctionCall::MutexLock => {
                self.call_mutex_lock(args, destination, &function_call_places);
            }
            FunctionCall::MutexNew => {
                self.call_mutex_new(destination, &function_call_places);
            }
            FunctionCall::ThreadJoin => {
                self.call_thread_join(args, &function_call_places);
            }
            FunctionCall::ThreadSpawn => {
                self.call_thread_spawn(args, destination, &function_call_places);
            }
            FunctionCall::Unwrap => {
                self.call_unwrap(args, destination, &function_call_places);
            }
        }
    }

    /// Handler for the case `FunctionCall::MirFunction`
    fn call_mir_function(
        &mut self,
        function_def_id: rustc_hir::def_id::DefId,
        function_call_places: FunctionPlaces,
    ) {
        let (start_place, end_place, _) = function_call_places;
        self.push_function_to_call_stack(function_def_id, start_place, end_place);
        self.translate_top_call_stack();
    }

    /// Handler for the case `FunctionCall::Foreign`
    fn call_foreign(&mut self, function_name: &str, function_call_places: &FunctionPlaces) {
        call_foreign_function(
            function_call_places,
            &foreign_call_transition_labels(function_name),
            &mut self.net,
        );
    }

    /// Handler for the case `FunctionCall::MutexLock`.
    fn call_mutex_lock(
        &mut self,
        args: &[rustc_middle::mir::Operand<'tcx>],
        destination: rustc_middle::mir::Place<'tcx>,
        function_call_places: &FunctionPlaces,
    ) {
        let transition_function_call = self
            .mutex_manager
            .translate_call_lock(function_call_places, &mut self.net);

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
        destination: rustc_middle::mir::Place<'tcx>,
        function_call_places: &FunctionPlaces,
    ) {
        self.mutex_manager
            .translate_call_new(function_call_places, &mut self.net);

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
        args: &[rustc_middle::mir::Operand<'tcx>],
        function_call_places: &FunctionPlaces,
    ) {
        let transition_function_call = self
            .thread_manager
            .translate_call_join(function_call_places, &mut self.net);

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
        args: &[rustc_middle::mir::Operand<'tcx>],
        destination: rustc_middle::mir::Place<'tcx>,
        function_call_places: &FunctionPlaces,
    ) {
        let transition_function_call = self
            .thread_manager
            .translate_call_spawn(function_call_places, &mut self.net);

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

    /// Handler for the the case `FunctionCall::ArcNew`.
    fn call_arc_new(
        &mut self,
        args: &[rustc_middle::mir::Operand<'tcx>],
        destination: rustc_middle::mir::Place<'tcx>,
        function_call_places: &FunctionPlaces,
    ) {
        self.arc_manager
            .translate_call_new(function_call_places, &mut self.net);

        let current_function = self.call_stack.peek_mut();
        ArcManager::translate_side_effects_new(
            args,
            destination,
            &mut current_function.memory,
            current_function.def_id,
            self.tcx,
        );
    }

    /// Handler for the the case `FunctionCall::Clone`.
    fn call_clone(
        &mut self,
        args: &[rustc_middle::mir::Operand<'tcx>],
        destination: rustc_middle::mir::Place<'tcx>,
        function_call_places: &FunctionPlaces,
    ) {
        self.arc_manager
            .translate_call_clone(function_call_places, &mut self.net);

        let current_function = self.call_stack.peek_mut();
        ArcManager::translate_side_effects_clone(
            args,
            destination,
            &mut current_function.memory,
            current_function.def_id,
            self.tcx,
        );
    }

    /// Handler for the the case `FunctionCall::Deref`.
    fn call_deref(
        &mut self,
        args: &[rustc_middle::mir::Operand<'tcx>],
        destination: rustc_middle::mir::Place<'tcx>,
        function_call_places: &FunctionPlaces,
    ) {
        self.arc_manager
            .translate_call_deref(function_call_places, &mut self.net);

        let current_function = self.call_stack.peek_mut();
        ArcManager::translate_side_effects_deref(
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
        destination: rustc_middle::mir::Place<'tcx>,
        function_call_places: &FunctionPlaces,
    ) {
        self.condvar_manager
            .translate_call_new(function_call_places, &mut self.net);

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
        args: &[rustc_middle::mir::Operand<'tcx>],
        function_call_places: &FunctionPlaces,
    ) {
        let notify_one_transition = self
            .condvar_manager
            .translate_call_notify_one(function_call_places, &mut self.net);

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
        function_call_places: &FunctionPlaces,
    ) {
        let wait_transitions = self
            .condvar_manager
            .translate_call_wait(function_call_places, &mut self.net);

        let current_function = self.call_stack.peek_mut();
        self.condvar_manager.translate_side_effects_wait(
            args,
            &wait_transitions,
            &mut self.net,
            &mut self.mutex_manager,
            &mut current_function.memory,
        );
    }

    /// Handler for the the case `FunctionCall::Unwrap`.
    fn call_unwrap(
        &mut self,
        args: &[rustc_middle::mir::Operand<'tcx>],
        destination: rustc_middle::mir::Place<'tcx>,
        function_call_places: &FunctionPlaces,
    ) {
        let current_function = self.call_stack.peek_mut();
        let self_ref = extract_nth_argument(args, 0);
        if current_function.memory.is_linked_to_lock_guard(self_ref) {
            current_function
                .memory
                .link_place_to_same_lock_guard(destination, self_ref);
        }
        // Reuse the `FunctionCall::Foreign` case. Nothing special to do.
        self.call_foreign("std::result::Result::<T, E>::unwrap", function_call_places);
    }
}
