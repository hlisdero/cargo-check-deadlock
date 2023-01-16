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

use netcrab::petri_net::TransitionRef;

pub struct ThreadSpan {
    /// The transition from which the thread branches off at the start.
    pub spawn_transition: TransitionRef,
    /// The definition ID that uniquely identifies the function run by the thread.
    pub thread_function_def_id: rustc_hir::def_id::DefId,
    /// The transition to which the thread joins in at the end.
    pub join_transition: Option<TransitionRef>,
}

impl ThreadSpan {
    /// Creates a new thread span without a join transition.
    /// The join transition must be set later.
    pub const fn new(
        spawn_transition: TransitionRef,
        thread_function_def_id: rustc_hir::def_id::DefId,
    ) -> Self {
        Self {
            spawn_transition,
            thread_function_def_id,
            join_transition: None,
        }
    }

    /// Sets the transition that models joining this thread.
    pub fn set_join_transition(&mut self, join_transition: TransitionRef) {
        self.join_transition = Some(join_transition);
    }
}
