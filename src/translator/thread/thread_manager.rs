//! Central structure to keep track of the threads in the code.
//!
//! The `ThreadManager` stores the threads discovered so far in the code.
//! It also performs the translation for each thread function.
//!
//! Once the translation of the main thread is over, each thread stored
//! here will be translated in order.

use crate::naming::thread_function_transition_label;
use crate::translator::special_function::call_foreign_function;
use crate::translator::sync::Memory;
use crate::translator::thread::ThreadSpan;
use crate::utils::place_to_local;
use crate::utils::{
    extract_def_id_of_called_function_from_operand,
    extract_self_reference_from_arguments_for_function_call,
};
use netcrab::petri_net::{PetriNet, PlaceRef, TransitionRef};
use std::collections::VecDeque;

#[derive(Default)]
pub struct ThreadManager {
    threads: VecDeque<ThreadSpan>,
    thread_join_counter: usize,
}

/// A wrapper type around the indexes to the elements in `VecDeque<ThreadSpan>`.
#[derive(Clone)]
pub struct ThreadRef(usize);

impl ThreadManager {
    /// Returns a new empty `ThreadManager`.
    pub fn new() -> Self {
        Self::default()
    }

    /// Translates a thread function call using the same logic as in `foreign_function_call`.
    /// Returns the transition that represents the function call.
    ///
    /// The naming depends:
    /// - For `std::thread::spawn` the numbering follows the order in which the threads are created.
    /// - For `std::thread::JoinHandle::<T>::join` a separate counter is incremented every time that the function is called.
    pub fn translate_function_call(
        &self,
        function_name: &str,
        start_place: &PlaceRef,
        end_place: &PlaceRef,
        net: &mut PetriNet,
    ) -> TransitionRef {
        let index = if function_name == "std::thread::spawn" {
            self.threads.len()
        } else {
            self.thread_join_counter
        };

        let transition_label = &thread_function_transition_label(function_name, index);
        // There is no cleanup place for the supported functions in `std::thread`.
        call_foreign_function(start_place, end_place, None, transition_label, net)
    }

    /// Translates the side effects for the methods of `std::thread`, i.e.,
    /// the specific logic of spawning a new thread and joining it.
    /// Receives a reference to the memory of the caller function to update
    /// and retrieve the mapping between local variables and thread references.
    pub fn translate_function_side_effects<'tcx>(
        &mut self,
        function_name: &str,
        args: &[rustc_middle::mir::Operand<'tcx>],
        return_value: rustc_middle::mir::Place,
        transition_function_call: TransitionRef,
        memory: &mut Memory,
        caller_function_def_id: rustc_hir::def_id::DefId,
        tcx: rustc_middle::ty::TyCtxt<'tcx>,
    ) {
        if function_name == "std::thread::spawn" {
            let function_to_be_run = args
                .get(0)
                .expect("BUG: `std::thread::spawn` should receive the function to be run");
            let thread_function_def_id = extract_def_id_of_called_function_from_operand(
                function_to_be_run,
                caller_function_def_id,
                tcx,
            );
            let thread_ref = self.add_thread_span(transition_function_call, thread_function_def_id);
            // The return value contains a new join handle. Link the local variable to it.
            let return_value_local = place_to_local(&return_value);
            memory.link_local_to_join_handle(return_value_local, thread_ref);
        } else if function_name == "std::thread::JoinHandle::<T>::join" {
            // Retrieve the join handle from the local variable passed to the function as an argument.
            let self_ref = extract_self_reference_from_arguments_for_function_call(args);
            let local_with_join_handle = place_to_local(&self_ref);
            let thread_ref = memory.get_linked_join_handle(local_with_join_handle);
            self.set_join_transition(thread_ref, transition_function_call);
        }
    }

    /// Adds a new thread span and returns a reference to it.
    pub fn add_thread_span(
        &mut self,
        spawn_transition: TransitionRef,
        thread_function_def_id: rustc_hir::def_id::DefId,
    ) -> ThreadRef {
        let index = self.threads.len();
        self.threads
            .push_front(ThreadSpan::new(spawn_transition, thread_function_def_id));
        ThreadRef(index)
    }

    /// Sets the join transition for a thread span.
    ///
    /// # Panics
    ///
    /// If the thread reference is invalid, then the function panics.
    pub fn set_join_transition(&mut self, thread_ref: &ThreadRef, join_transition: TransitionRef) {
        let thread_span = self.threads.get_mut(thread_ref.0).expect(
            "BUG: The thread reference should be a valid index for the vector of thread spans",
        );
        thread_span.set_join_transition(join_transition);
    }

    /// Removes the last element from the threads vector and returns it, or `None` if it is empty.
    pub fn pop_thread(&mut self) -> Option<ThreadSpan> {
        self.threads.pop_front()
    }
}
