//! Submodule for defining Petri net places and transitions
//! common to all functions handled by the translator.

use crate::data_structures::petri_net_interface::{PlaceRef, TransitionRef};

/// An enum containing the Petri net places for a function call.
pub enum Places {
    /// A basic function with a start and an end place.
    /// These places represent where the function starts and ends in the Petri net.
    Basic {
        start_place: PlaceRef,
        end_place: PlaceRef,
    },
    /// A function with a start, an end place, and a cleanup place.
    /// These places represent where the function starts and ends in the Petri net.
    /// The cleanup place is an alternative termination path taken in case of errors.
    WithCleanup {
        start_place: PlaceRef,
        end_place: PlaceRef,
        cleanup_place: PlaceRef,
    },
}

/// An enum containing the Petri net transitions for a function call.
pub enum Transitions {
    /// A basic function modelled with a single transition.
    Basic { transition: TransitionRef },
    /// A function modelled by two transitions.
    /// One for the case that the function executes sucessfully
    /// and the other for the cleanup in case of errors.
    WithCleanup {
        transition: TransitionRef,
        cleanup_transition: TransitionRef,
    },
}

impl Transitions {
    /// Returns the default transition for the function call.
    pub const fn get_transition(&self) -> &TransitionRef {
        match self {
            Self::Basic { transition } | Self::WithCleanup { transition, .. } => transition,
        }
    }
}
