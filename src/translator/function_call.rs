//! Submodule for defining the function calls supported by the translator and
//! the specific handler methods for every one of them.

use crate::naming::arc::function_transition_label;
use crate::naming::function::foreign_call_transition_label;
use crate::translator::multithreading::{is_thread_join, is_thread_spawn};
use crate::translator::special_function::{call_foreign_function, is_foreign_function};
use crate::translator::sync::{
    detect_deref_arc_with_mutex, detect_mutex_inside_arc_new, is_arc_new, is_deref, is_mutex_lock,
    is_mutex_new,
};
use crate::translator::Translator;
use netcrab::petri_net::PlaceRef;

/// Types of function calls that the translator supports.
pub enum FunctionCall {
    /// Call to `std::sync::Arc::<T>::new`
    /// Non-recursive call for the translation process.
    ArcNew,
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
}

impl FunctionCall {
    /// Creates a new function call depending on the specific function that will be called.
    pub fn new(function_def_id: rustc_hir::def_id::DefId, tcx: rustc_middle::ty::TyCtxt) -> Self {
        let function_name = tcx.def_path_str(function_def_id);

        if is_arc_new(&function_name) {
            return Self::ArcNew;
        }
        if is_deref(&function_name) {
            return Self::Deref;
        }
        if is_mutex_new(&function_name) {
            return Self::MutexNew;
        }
        if is_mutex_lock(&function_name) {
            return Self::MutexLock;
        }
        if is_thread_spawn(&function_name) {
            return Self::ThreadSpawn;
        }
        if is_thread_join(&function_name) {
            return Self::ThreadJoin;
        }
        // Default case for standard and core library calls
        if is_foreign_function(function_def_id, tcx) {
            return Self::Foreign;
        }
        // Default case: A function with MIR representation
        Self::MirFunction
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
        place_refs_for_function_call: (PlaceRef, PlaceRef, Option<PlaceRef>),
    ) {
        let function_name = self.tcx.def_path_str(function_def_id);
        let (start_place, end_place, cleanup_place) = place_refs_for_function_call;

        match function_call {
            FunctionCall::ArcNew => {
                self.call_arc_new(args, destination, &start_place, &end_place);
            }
            FunctionCall::Deref => {
                self.call_deref(args, destination, &start_place, &end_place, cleanup_place);
            }
            FunctionCall::MirFunction => {
                self.call_mir_function(function_def_id, start_place, end_place);
            }
            FunctionCall::Foreign => {
                self.call_foreign(&function_name, &start_place, &end_place, cleanup_place);
            }
            FunctionCall::MutexLock => {
                self.call_mutex_lock(args, destination, &start_place, &end_place, cleanup_place);
            }
            FunctionCall::MutexNew => {
                self.call_mutex_new(destination, &start_place, &end_place, cleanup_place);
            }
            FunctionCall::ThreadJoin => {
                self.call_thread_join(args, &start_place, &end_place);
            }
            FunctionCall::ThreadSpawn => {
                self.call_thread_spawn(args, destination, &start_place, &end_place);
            }
        }
    }

    /// Handler for the case `FunctionCall::MirFunction`
    pub fn call_mir_function(
        &mut self,
        function_def_id: rustc_hir::def_id::DefId,
        start_place: PlaceRef,
        end_place: PlaceRef,
    ) {
        self.push_function_to_call_stack(function_def_id, start_place, end_place);
        self.translate_top_call_stack();
    }

    /// Handler for the case `FunctionCall::Foreign`
    pub fn call_foreign(
        &mut self,
        function_name: &str,
        start_place: &PlaceRef,
        end_place: &PlaceRef,
        cleanup_place: Option<PlaceRef>,
    ) {
        let transition_label = &foreign_call_transition_label(function_name);
        call_foreign_function(
            start_place,
            end_place,
            cleanup_place,
            transition_label,
            &mut self.net,
        );
    }

    /// Handler for the case `FunctionCall::MutexLock`.
    pub fn call_mutex_lock(
        &mut self,
        args: &[rustc_middle::mir::Operand<'tcx>],
        destination: rustc_middle::mir::Place<'tcx>,
        start_place: &PlaceRef,
        end_place: &PlaceRef,
        cleanup_place: Option<PlaceRef>,
    ) {
        let transition_function_call = self.mutex_manager.translate_call_lock(
            start_place,
            end_place,
            cleanup_place,
            &mut self.net,
        );

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
    pub fn call_mutex_new(
        &mut self,
        destination: rustc_middle::mir::Place<'tcx>,
        start_place: &PlaceRef,
        end_place: &PlaceRef,
        cleanup_place: Option<PlaceRef>,
    ) {
        self.mutex_manager
            .translate_call_new(start_place, end_place, cleanup_place, &mut self.net);

        let current_function = self.call_stack.peek_mut();
        self.mutex_manager.translate_side_effects_new(
            destination,
            &mut self.net,
            &mut current_function.memory,
        );
    }

    /// Handler for the case `FunctionCall::ThreadJoin`.
    pub fn call_thread_join(
        &mut self,
        args: &[rustc_middle::mir::Operand<'tcx>],
        start_place: &PlaceRef,
        end_place: &PlaceRef,
    ) {
        let transition_function_call =
            self.thread_manager
                .translate_call_join(start_place, end_place, &mut self.net);

        let current_function = self.call_stack.peek();
        self.thread_manager.translate_side_effects_join(
            args,
            transition_function_call,
            &current_function.memory,
        );
    }

    /// Handler for the case `FunctionCall::ThreadSpawn`.
    pub fn call_thread_spawn(
        &mut self,
        args: &[rustc_middle::mir::Operand<'tcx>],
        destination: rustc_middle::mir::Place<'tcx>,
        start_place: &PlaceRef,
        end_place: &PlaceRef,
    ) {
        let transition_function_call =
            self.thread_manager
                .translate_call_spawn(start_place, end_place, &mut self.net);

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

    /// Handler for the function `std::sync::Arc::<T>::new`
    pub fn call_arc_new(
        &mut self,
        args: &[rustc_middle::mir::Operand<'tcx>],
        destination: rustc_middle::mir::Place<'tcx>,
        start_place: &PlaceRef,
        end_place: &PlaceRef,
    ) {
        let current_function = self.call_stack.peek_mut();
        let body = self.tcx.optimized_mir(current_function.def_id);
        let first_argument = args
            .get(0)
            .expect("BUG: `std::sync::Arc::<T>::new` should receive at least one argument");

        if let Some((return_value_local, local_with_mutex)) =
            detect_mutex_inside_arc_new(first_argument, destination, body)
        {
            current_function
                .memory
                .link_local_to_same_mutex(return_value_local, local_with_mutex);
        }
        let function_name = "std::sync::Arc::<T>::new";
        let count = self.function_counter.get_count(function_name);
        self.function_counter.increment(function_name.to_string());
        let transition_label = &function_transition_label(function_name, count);
        call_foreign_function(
            start_place,
            end_place,
            None,
            transition_label,
            &mut self.net,
        );
    }

    /// Handler for the function `std::ops::Deref::deref`
    pub fn call_deref(
        &mut self,
        args: &[rustc_middle::mir::Operand<'tcx>],
        destination: rustc_middle::mir::Place<'tcx>,
        start_place: &PlaceRef,
        end_place: &PlaceRef,
        cleanup_place: Option<PlaceRef>,
    ) {
        let current_function = self.call_stack.peek_mut();
        let body = self.tcx.optimized_mir(current_function.def_id);
        let first_argument = args
            .get(0)
            .expect("BUG: `std::ops::Deref` should receive at least one argument");

        if let Some((return_value_local, local_with_mutex)) =
            detect_deref_arc_with_mutex(first_argument, destination, body)
        {
            current_function
                .memory
                .link_local_to_same_mutex(return_value_local, local_with_mutex);
        }

        let function_name = "std::ops::Deref";
        let count = self.function_counter.get_count(function_name);
        self.function_counter.increment(function_name.to_string());
        let transition_label = &function_transition_label(function_name, count);
        call_foreign_function(
            start_place,
            end_place,
            cleanup_place,
            transition_label,
            &mut self.net,
        );
    }
}
