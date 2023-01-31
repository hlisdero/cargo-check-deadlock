//! Central structure to keep track of the threads in the code.
//!
//! The `ThreadManager` stores the threads discovered so far in the code.
//! It also performs the translation for each thread function.
//!
//! Once the translation of the main thread is over, each thread stored
//! here will be translated in order.

use super::Thread;
use crate::naming::thread::{join_transition_labels, spawn_transition_labels};
use crate::translator::mir_function::{Memory, MutexEntries};
use crate::translator::special_function::call_foreign_function;
use crate::utils::{extract_def_id_of_called_function_from_operand, extract_nth_argument};
use netcrab::petri_net::{PetriNet, PlaceRef, TransitionRef};
use std::collections::VecDeque;

#[derive(Default)]
pub struct ThreadManager<'tcx> {
    threads: VecDeque<Thread<'tcx>>,
    thread_join_counter: usize,
}

/// A wrapper type around the indexes to the elements in `VecDeque<ThreadSpan>`.
#[derive(Clone)]
pub struct ThreadRef(usize);

impl<'tcx> ThreadManager<'tcx> {
    /// Returns a new empty `ThreadManager`.
    pub fn new() -> Self {
        Self::default()
    }

    /// Translates a call to `std::thread::spawn` using
    /// the same representation as in `foreign_function_call`.
    /// The labelling follows the numbering of the labels of the threads.
    /// Returns the transition that represents the function call.
    pub fn translate_call_spawn(
        &self,
        start_place: &PlaceRef,
        end_place: &PlaceRef,
        cleanup_place: Option<PlaceRef>,
        net: &mut PetriNet,
    ) -> TransitionRef {
        let index = self.threads.len();
        call_foreign_function(
            start_place,
            end_place,
            cleanup_place,
            &spawn_transition_labels(index),
            net,
        )
    }

    /// Translates a call to `std::thread::JoinHandle::<T>::join` using
    /// the same representation as in `foreign_function_call`.
    /// A separate counter is incremented every time that
    /// the function is called to generate a unique label.
    /// Returns the transition that represents the function call.
    pub fn translate_call_join(
        &mut self,
        start_place: &PlaceRef,
        end_place: &PlaceRef,
        cleanup_place: Option<PlaceRef>,
        net: &mut PetriNet,
    ) -> TransitionRef {
        let index = self.thread_join_counter;
        self.thread_join_counter += 1;
        call_foreign_function(
            start_place,
            end_place,
            cleanup_place,
            &join_transition_labels(index),
            net,
        )
    }

    /// Translates the side effects for `std::thread::spawn` i.e.,
    /// the specific logic of spawning a new thread.
    /// Receives a reference to the memory of the caller function to
    /// link the return local variable to the new join handle.
    pub fn translate_side_effects_spawn(
        &mut self,
        args: &[rustc_middle::mir::Operand<'tcx>],
        return_value: rustc_middle::mir::Place<'tcx>,
        transition_function_call: TransitionRef,
        memory: &mut Memory<'tcx>,
        caller_function_def_id: rustc_hir::def_id::DefId,
        tcx: rustc_middle::ty::TyCtxt<'tcx>,
    ) {
        let function_to_be_run = args
            .get(0)
            .expect("BUG: `std::thread::spawn` should receive the function to be run");
        let thread_function_def_id = extract_def_id_of_called_function_from_operand(
            function_to_be_run,
            caller_function_def_id,
            tcx,
        );

        let closure_for_spawn = extract_nth_argument(args, 0);
        let mutexes = memory.find_mutexes_linked_to_place(closure_for_spawn);

        let thread_ref =
            self.add_thread_span(transition_function_call, thread_function_def_id, mutexes);
        // The return value contains a new join handle. Link the local variable to it.
        memory.link_place_to_join_handle(return_value, thread_ref);
    }

    /// Translates the side effects for `std::thread::JoinHandle::<T>::join` i.e.,
    /// the specific logic of joining an existing thread.
    /// Receives a reference to the memory of the caller function to retrieve
    /// the join handle linked to the local variable.
    pub fn translate_side_effects_join(
        &mut self,
        args: &[rustc_middle::mir::Operand<'tcx>],
        transition_function_call: TransitionRef,
        memory: &Memory<'tcx>,
    ) {
        // Retrieve the join handle from the local variable passed to the function as an argument.
        let self_ref = extract_nth_argument(args, 0);
        let thread_ref = memory.get_linked_join_handle(&self_ref);
        self.set_join_transition(thread_ref, transition_function_call);
    }

    /// Adds a new thread span and returns a reference to it.
    fn add_thread_span(
        &mut self,
        spawn_transition: TransitionRef,
        thread_function_def_id: rustc_hir::def_id::DefId,
        mutexes: MutexEntries<'tcx>,
    ) -> ThreadRef {
        let index = self.threads.len();
        self.threads.push_front(Thread::new(
            spawn_transition,
            thread_function_def_id,
            mutexes,
            index,
        ));
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
    pub fn pop_thread(&mut self) -> Option<Thread<'tcx>> {
        self.threads.pop_front()
    }
}
