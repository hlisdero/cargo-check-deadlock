//! Representation of a thread to be translated later.
//!
//! The thread involves a translation subprocess that connects
//! to the main Petri net in two places:
//! - The start place i.e, when `std::thread::spawn` is called.
//! - The end place i.e, when `std::thread::JoinHandle::<T>::join` is called.
//!
//! The thread starts after the call to `std::thread::spawn`.
//! The transition that represents this function call produces a second token
//! and places it in the new thread's start place.
//! This token models the execution flow of the new thread.
//!
//! The thread end place is connected to the transition that models
//! the call to `std::thread::JoinHandle::<T>::join`.
//!
//! The Petri net for the new thread "spans" between the two transitions.
//! We cannot connect this net to the net of the spawning thread until the
//! call to `std::thread::JoinHandle::<T>::join` is translated.
//! Therefore we store the basic information we need
//! to translate the thread function and defer the translation.
//! The function executed by the thread is translated to a Petri net just as any other.

use crate::data_structures::petri_net_interface::{
    add_arc_place_transition, add_arc_transition_place,
};
use crate::data_structures::petri_net_interface::{PetriNet, PlaceRef, TransitionRef};
use crate::naming::thread::{end_place_label, start_place_label};
use crate::translator::mir_function::{Entries, Memory};
use crate::translator::sync::{CondvarRef, MutexGuardRef, MutexRef, ThreadRef};
use crate::utils::check_substring_in_place_type;

pub struct Thread {
    /// The transition from which the thread branches off at the start.
    spawn_transition: TransitionRef,
    /// The definition ID that uniquely identifies the function run by the thread.
    thread_function_def_id: rustc_hir::def_id::DefId,
    /// The mutexes passed to the thread.
    mutexes: Entries<MutexRef>,
    /// The mutex guards passed to the thread.
    mutex_guards: Entries<MutexGuardRef>,
    /// The join handles passed to the thread.
    join_handles: Entries<ThreadRef>,
    /// The condition variables passed to the thread.
    condvars: Entries<CondvarRef>,
    /// The transition to which the thread joins in at the end.
    join_transition: Option<TransitionRef>,
    /// An index to identify the thread.
    pub index: usize,
}

impl Thread {
    /// Creates a new thread without a join transition.
    /// The join transition must be set later.
    pub const fn new(
        spawn_transition: TransitionRef,
        thread_function_def_id: rustc_hir::def_id::DefId,
        mutexes: Entries<MutexRef>,
        mutex_guards: Entries<MutexGuardRef>,
        join_handles: Entries<ThreadRef>,
        condvars: Entries<CondvarRef>,
        index: usize,
    ) -> Self {
        Self {
            spawn_transition,
            thread_function_def_id,
            mutexes,
            mutex_guards,
            join_handles,
            condvars,
            join_transition: None,
            index,
        }
    }

    /// Sets the transition that models joining this thread.
    pub fn set_join_transition(&mut self, join_transition: TransitionRef) {
        self.join_transition = Some(join_transition);
    }

    /// Prepares the thread for translation.
    /// Adds a start and end place for the thread to the Petri net.
    /// Connects the spawn transition to the start place and the end place to the join transition (if available).
    /// Returns a 3-tuple containing the definition ID, the start place and the end place.
    pub fn prepare_for_translation(
        &self,
        net: &mut PetriNet,
    ) -> (rustc_hir::def_id::DefId, PlaceRef, PlaceRef) {
        let thread_start_place = net.add_place(&start_place_label(self.index));
        let thread_end_place = net.add_place(&end_place_label(self.index));

        add_arc_transition_place(net, &self.spawn_transition, &thread_start_place);
        if let Some(join_transition) = &self.join_transition {
            add_arc_place_transition(net, &thread_end_place, join_transition);
        }

        (
            self.thread_function_def_id,
            thread_start_place,
            thread_end_place,
        )
    }

    /// Moves the memory entries corresponding to the synchronization variables to the new function's memory.
    /// Supports moving mutexes and condition variables.
    /// Checks the debug info to detect places containing a synchronization variable passed to the new thread.
    /// We are only interested in places of the form `_1.X` since `std::thread::spawn` only receives one argument.
    /// <https://doc.rust-lang.org/stable/nightly-rustc/rustc_middle/mir/struct.VarDebugInfo.html>
    ///
    /// # Examples
    ///
    /// The following line in the MIR output indicates that `_1.0` contains a mutex.
    /// `debug copy_data => (_1.0: std::sync::Arc<std::sync::Mutex<i32>>)`
    pub fn move_sync_variables<'tcx>(
        &mut self,
        memory: &mut Memory<'tcx>,
        tcx: rustc_middle::ty::TyCtxt<'tcx>,
    ) {
        let body = tcx.optimized_mir(self.thread_function_def_id);
        for debug_info in &body.var_debug_info {
            let rustc_middle::mir::VarDebugInfoContents::Place(place) = debug_info.value else {
                // Not interested in the other variants of `VarDebugInfoContents`
                continue;
            };
            if place.local != rustc_middle::mir::Local::from(1u32) {
                // Not interested in locals other that `_1.X`
                continue;
            }
            if check_substring_in_place_type(
                &place,
                "std::sync::Mutex<",
                self.thread_function_def_id,
                tcx,
            ) {
                let mutex_ref = self.mutexes.pop().expect(
                    "BUG: The thread function receives more mutexes than the ones detected",
                );
                memory.link_place_to_mutex(place, mutex_ref);
            }
            if check_substring_in_place_type(
                &place,
                "std::sync::MutexGuard<",
                self.thread_function_def_id,
                tcx,
            ) {
                let mutex_guard_ref = self.mutex_guards.pop().expect(
                    "BUG: The thread function receives more mutex guards than the ones detected",
                );
                memory.link_place_to_mutex_guard(place, mutex_guard_ref);
            }
            if check_substring_in_place_type(
                &place,
                "std::thread::JoinHandle<",
                self.thread_function_def_id,
                tcx,
            ) {
                let thread_ref = self.join_handles.pop().expect(
                    "BUG: The thread function receives more join handles than the ones detected",
                );
                memory.link_place_to_join_handle(place, thread_ref);
            }
            if check_substring_in_place_type(
                &place,
                "std::sync::Condvar",
                self.thread_function_def_id,
                tcx,
            ) {
                let condvar_ref = self.condvars.pop().expect(
                "BUG: The thread function receives more condition variables than the ones detected",
            );
                memory.link_place_to_condvar(place, condvar_ref);
            }
        }
    }
}
