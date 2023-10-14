//! Submodule for defining Petri net places and transitions
//! common to all functions handled by the translator.
//!
//! It also defines the different postprocessing tasks
//! and their respective priorities.

use crate::data_structures::petri_net_interface::{PlaceRef, TransitionRef};
use crate::translator::MutexRef;

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
    // Convert the enum to its most basic form `Basic`.
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

    // Returns the start and the end place as a tuple, consuming the enum.
    pub fn get_start_end_place(self) -> (PlaceRef, PlaceRef) {
        match self {
            Self::Basic {
                start_place,
                end_place,
            }
            | Self::WithCleanup {
                start_place,
                end_place,
                ..
            } => (start_place, end_place),
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

/// An enum for different tasks performed after translating all threads.
/// The tasks are ordered by priority.
#[derive(PartialEq, Eq)]
pub enum PostprocessingTask {
    LinkMutexToCondvar {
        priority: u8,
        index: usize,
        start_place: PlaceRef,
        end_place: PlaceRef,
        wait_start: TransitionRef,
    },
    NewMutex {
        priority: u8,
        mutex_ref: MutexRef,
    },
}

impl std::cmp::PartialOrd for PostprocessingTask {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl std::cmp::Ord for PostprocessingTask {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.get_priority().cmp(&other.get_priority())
    }
}

impl PostprocessingTask {
    /// Creates a new instance of `PostprocessingTask::LinkMutexToCondvar`.
    pub fn link_mutex_to_condvar(
        index: usize,
        start_place: PlaceRef,
        end_place: PlaceRef,
        wait_start: TransitionRef,
    ) -> Self {
        Self::LinkMutexToCondvar {
            priority: 1,
            index,
            start_place,
            end_place,
            wait_start,
        }
    }

    /// Creates a new instance of `PostprocessingTask::NewMutex`.
    pub fn new_mutex(mutex_ref: MutexRef) -> Self {
        Self::NewMutex {
            priority: 2,
            mutex_ref,
        }
    }

    /// Returns the priority for any of the enum variants.
    fn get_priority(&self) -> u8 {
        match self {
            Self::LinkMutexToCondvar { priority, .. } | Self::NewMutex { priority, .. } => {
                *priority
            }
        }
    }
}
