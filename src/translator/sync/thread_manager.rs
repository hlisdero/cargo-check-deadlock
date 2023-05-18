//! Central structure to keep track of the threads in the code.
//!
//! The `ThreadManager` stores the threads discovered so far in the code.
//! It also performs the translation for each thread function.
//!
//! Once the translation of the main thread is over, each thread stored
//! here will be translated in order.

use super::{CondvarRef, MutexGuardRef, MutexRef, Thread};
use crate::data_structures::petri_net_interface::TransitionRef;
use crate::translator::mir_function::{Entries, Memory};
use crate::utils::{
    extract_closure, extract_def_id_of_called_function_from_operand, extract_nth_argument_as_place,
};
use log::{debug, info};
use std::collections::VecDeque;

#[derive(Default)]
pub struct ThreadManager {
    threads: VecDeque<Thread>,
}

/// A wrapper type around the indexes to the elements in `VecDeque<ThreadSpan>`.
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct ThreadRef(usize);

impl ThreadManager {
    /// Returns a new empty `ThreadManager`.
    pub fn new() -> Self {
        Self::default()
    }

    /// Translates the side effects for `std::thread::spawn` i.e.,
    /// the specific logic of spawning a new thread.
    /// Receives a reference to the memory of the caller function to
    /// link the return local variable to the new join handle.
    pub fn translate_side_effects_spawn<'tcx>(
        &mut self,
        args: &[rustc_middle::mir::Operand<'tcx>],
        return_value: rustc_middle::mir::Place<'tcx>,
        function_call_transition: TransitionRef,
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

        let closure_for_spawn = extract_closure(args);
        let memory_entries = Self::find_sync_variables(closure_for_spawn, memory);

        let thread_ref = self.add_thread(
            function_call_transition,
            thread_function_def_id,
            memory_entries,
        );
        // The return value contains a new join handle. Link the local variable to it.
        memory.link_place_to_join_handle(return_value, thread_ref);
        debug!("NEW JOIN HANDLE: {return_value:?}");
    }

    /// Translates the side effects for `std::thread::JoinHandle::<T>::join` i.e.,
    /// the specific logic of joining an existing thread.
    /// Receives a reference to the memory of the caller function to retrieve
    /// the join handle linked to the local variable.
    pub fn translate_side_effects_join<'tcx>(
        &mut self,
        args: &[rustc_middle::mir::Operand<'tcx>],
        function_call_transition: TransitionRef,
        memory: &Memory<'tcx>,
    ) {
        // Retrieve the join handle from the local variable passed to the function as an argument.
        let self_ref = extract_nth_argument_as_place(args, 0).expect(
            "BUG: `std::thread::JoinHandle::<T>::join` should receive the self reference as a place",
        );
        let thread_ref = memory.get_linked_join_handle(&self_ref);
        self.set_join_transition(*thread_ref, function_call_transition);
    }

    /// Adds a new thread and returns a reference to it.
    fn add_thread(
        &mut self,
        spawn_transition: TransitionRef,
        thread_function_def_id: rustc_hir::def_id::DefId,
        memory_entries: (
            Entries<MutexRef>,
            Entries<MutexGuardRef>,
            Entries<ThreadRef>,
            Entries<CondvarRef>,
        ),
    ) -> ThreadRef {
        let index = self.threads.len();
        self.threads.push_back(Thread::new(
            spawn_transition,
            thread_function_def_id,
            memory_entries.0,
            memory_entries.1,
            memory_entries.2,
            memory_entries.3,
            index,
        ));
        info!("Found thread {index} and pushed it to the back of the thread translation queue");
        ThreadRef(index)
    }

    /// Sets the join transition for a thread.
    ///
    /// # Panics
    ///
    /// If the thread reference is invalid, then the function panics.
    pub fn set_join_transition(&mut self, thread_ref: ThreadRef, join_transition: TransitionRef) {
        let thread = self
            .threads
            .get_mut(thread_ref.0)
            .expect("BUG: The thread reference should be a valid index for the vector of threads");
        thread.set_join_transition(join_transition);
        info!("Found join call for thread {}", thread.index);
    }

    /// Removes the last element from the threads vector and returns it, or `None` if it is empty.
    pub fn pop_thread(&mut self) -> Option<Thread> {
        self.threads.pop_front()
    }

    /// Finds sync variables captured by the closure for a new thread.
    /// Returns the memory entries for each sync variable type that should be re-mapped in the new thread's memory.
    ///
    /// If the closure is `None` (no variables were captured, it is a `ZeroSizedType`),
    /// then return empty vectors for the memory entries.
    fn find_sync_variables<'tcx>(
        closure: Option<rustc_middle::mir::Place<'tcx>>,
        memory: &mut Memory<'tcx>,
    ) -> (
        Entries<MutexRef>,
        Entries<MutexGuardRef>,
        Entries<ThreadRef>,
        Entries<CondvarRef>,
    ) {
        closure.map_or_else(
            || {
                let mutexes = vec![];
                let mutex_guards = vec![];
                let join_handles = vec![];
                let condvars = vec![];
                (mutexes, mutex_guards, join_handles, condvars)
            },
            |place| {
                let mutexes = memory.find_mutexes_linked_to_place(place);
                let mutex_guards = memory.find_mutex_guards_linked_to_place(place);
                let join_handles = memory.find_join_handles_linked_to_place(place);
                let condvars = memory.find_condvars_linked_to_place(place);
                (mutexes, mutex_guards, join_handles, condvars)
            },
        )
    }
}
