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

use log::info;
use std::cell::OnceCell;

use crate::data_structures::petri_net_interface::{PetriNet, PlaceRef, TransitionRef};
use crate::data_structures::petri_net_interface::{
    add_arc_place_transition, add_arc_transition_place,
};
use crate::naming::thread::{end_place_label, start_place_label};
use crate::translator::function::Places;
use crate::translator::mir_function::MirFunction;
use crate::translator::mir_function::memory::Memory;
use crate::translator::special_function::call_foreign_function;
use crate::utils::extract_nth_argument_as_place;

pub struct Thread {
    /// The transition from which the thread branches off at the start.
    spawn_transition: TransitionRef,
    /// The MIR function that the thread will execute
    mir_function: MirFunction,
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
        mir_function: MirFunction,
        index: usize,
    ) -> Self {
        Self {
            spawn_transition,
            mir_function,
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

    /// Connects:
    /// - The spawn transition to the start place
    /// - The end place to the join transition (if available).
    pub fn create_arcs_for_transitions(&self, net: &mut PetriNet) {
        add_arc_transition_place(net, &self.spawn_transition, &self.mir_function.start_place);
        if let Some(join_transition) = self.join_transition.get() {
            add_arc_place_transition(net, &self.mir_function.end_place, join_transition);
        }
    }

    /// Creates the correctly labelled places for the thread's function.
    /// Returns the references to the start and end place.
    pub fn create_start_and_end_places(net: &mut PetriNet, index: usize) -> (PlaceRef, PlaceRef) {
        let start_place = net.add_place(&start_place_label(index));
        let end_place = net.add_place(&end_place_label(index));
        (start_place, end_place)
    }

    /// Returns the MIR function of the thread, consuming the thread in the process
    pub fn take_mir_function(self) -> MirFunction {
        self.mir_function
    }

    /// Returns a copy of the end place of the thread's MIR function
    pub fn clone_end_place(&self) -> PlaceRef {
        self.mir_function.end_place.clone()
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
