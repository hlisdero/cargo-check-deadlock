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

use log::{debug, info};
use std::cell::OnceCell;

use crate::data_structures::petri_net_interface::{
    add_arc_place_transition, add_arc_transition_place,
};
use crate::data_structures::petri_net_interface::{PetriNet, PlaceRef, TransitionRef};
use crate::naming::thread::{end_place_label, start_place_label};
use crate::translator::function::Places;
use crate::translator::mir_function::memory::{Memory, Value};
use crate::translator::special_function::call_foreign_function;
use crate::utils::extract_nth_argument_as_place;

pub struct Thread {
    /// The transition from which the thread branches off at the start.
    spawn_transition: TransitionRef,
    /// The definition ID that uniquely identifies the function run by the thread.
    def_id: rustc_hir::def_id::DefId,
    /// The aggregate value containing the sync variables passed to the thread.
    aggregate: Value,
    /// The transition to which the thread joins in at the end.
    join_transition: OnceCell<TransitionRef>,
    /// An index to identify the thread.
    pub index: usize,
}

impl std::cmp::PartialEq for Thread {
    fn eq(&self, other: &Self) -> bool {
        self.index == other.index
    }
}

impl std::cmp::Eq for Thread {}

impl Thread {
    /// Creates a new thread without a join transition.
    /// The join transition must be set later.
    pub const fn new(
        spawn_transition: TransitionRef,
        thread_function_def_id: rustc_hir::def_id::DefId,
        aggregate: Value,
        index: usize,
    ) -> Self {
        Self {
            spawn_transition,
            def_id: thread_function_def_id,
            aggregate,
            join_transition: OnceCell::new(),
            index,
        }
    }

    /// Sets the transition that models joining this thread.
    pub fn set_join_transition(&self, join_transition: TransitionRef) {
        let result = self.join_transition.set(join_transition);
        assert!(
            result.is_ok(),
            "BUG: The join transition of a thread may only be set once"
        );
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
        if let Some(join_transition) = self.join_transition.get() {
            add_arc_place_transition(net, &thread_end_place, join_transition);
        }

        (self.def_id, thread_start_place, thread_end_place)
    }

    /// Moves the aggregated value containing the sync variables to the new function's memory in local `_1`.
    /// We are only interested in places of the form `_1.X` since `std::thread::spawn` only receives one argument.
    /// <https://doc.rust-lang.org/stable/nightly-rustc/rustc_middle/mir/struct.VarDebugInfo.html>
    ///
    /// # Examples
    ///
    /// The following line in the MIR output indicates that `_1.0` contains a mutex.
    /// `debug copy_data => (_1.0: std::sync::Arc<std::sync::Mutex<i32>>)`
    pub fn move_sync_variables(&self, memory: &mut Memory) {
        let place: rustc_middle::mir::Place<'_> = rustc_middle::mir::Place {
            local: rustc_middle::mir::Local::from_usize(1),
            projection: rustc_middle::ty::List::empty(),
        };
        memory.link(place, self.aggregate.clone());
        debug!(
            "MOVED AGGREGATE VALUE {place:?} WITH SYNC VARIABLES {:?} TO THE THREAD {}",
            self.aggregate, self.index
        );
        debug!("{memory:?}");
    }
}

/// Call to `std::thread::JoinHandle::<T>::join`.
/// Non-recursive call for the translation process.
///
/// - Retrieves the join handle linked to the first argument (the self reference).
/// - Sets the join transition for the thread.
///
/// In some cases, the `std::thread::JoinHandle::<T>::join` function contains a cleanup target.
/// This target is not called in practice but creates trouble for deadlock detection.
/// For instance, a thread that never returns will not cause a deadlock
/// when joining it because the call could take the unwind path.
/// In conclusion: Ignore the cleanup place, do not model it. Assume `join` never unwinds.
pub fn call_join(
    function_name: &str,
    index: usize,
    args: &[rustc_span::source_map::Spanned<rustc_middle::mir::Operand>],
    places: Places,
    net: &mut PetriNet,
    memory: &Memory,
) {
    let places = places.ignore_cleanup_place();
    let transitions = call_foreign_function(function_name, index, places, net);
    let transition = transitions.default();
    // Retrieve the join handle from the local variable passed to the function as an argument.
    let self_ref = extract_nth_argument_as_place(args, 0).unwrap_or_else(|| {
        panic!("BUG: `{function_name}` should receive the self reference as a place")
    });
    let thread_ref = memory.get_join_handle(&self_ref);
    thread_ref.set_join_transition(transition);
    info!("Found join call for thread {}", thread_ref.index);
}
