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

impl Places {
    pub fn ignore_cleanup_place(self) -> Self {
        match self {
            Self::Basic { .. } => self,
            Self::WithCleanup {
                start_place,
                end_place,
                ..
            } => Self::Basic {
                start_place,
                end_place,
            },
        }
    }
}

/// An enum containing the Petri net transitions for a function call.
pub enum Transitions {
    /// A basic function modelled with a single transition.
    Basic { default: TransitionRef },
    /// A function modelled by two transitions.
    /// One for the case that the function executes successfully
    /// and the other for the cleanup in case of errors.
    WithCleanup {
        default: TransitionRef,
        cleanup: TransitionRef,
    },
}

impl Transitions {
    /// Returns the default transition for the function call as a reference
    pub const fn get_default(&self) -> &TransitionRef {
        match self {
            Self::Basic { default } | Self::WithCleanup { default, .. } => default,
        }
    }

    /// Returns the default transition for the function call consuming the enum value.
    pub fn default(self) -> TransitionRef {
        match self {
            Self::Basic { default } | Self::WithCleanup { default, .. } => default,
        }
    }
}
