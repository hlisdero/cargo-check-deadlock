//! Representation of a memory value that is of interest to the translator
//!
//! For convenience, it also defines the types for the sync variables references

use crate::translator::sync::{Condvar, Mutex, MutexGuard, Thread};

/// A mutex reference is just a shared pointer to the mutex.
pub type MutexRef = std::rc::Rc<Mutex>;

/// A mutex guard reference is just a shared pointer to the mutex guard.
pub type MutexGuardRef = std::rc::Rc<MutexGuard>;

/// A condvar reference is just a shared pointer to the condition variable.
pub type CondvarRef = std::rc::Rc<Condvar>;

/// A thread reference is just a shared pointer to the thread.
pub type ThreadRef = std::rc::Rc<Thread>;

/// Possible values that can be stored in the `Memory`.
/// A place will be mapped to one of these.
#[derive(Eq, PartialEq, Clone)]
pub enum Value {
    None,
    Single(Single),
    Aggregate(Vec<Self>),
}

impl std::fmt::Display for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Single(single_value) => write!(f, "{single_value}"),
            Self::Aggregate(values) => {
                let formatted_values: Vec<String> = values
                    .iter()
                    .map(std::string::ToString::to_string)
                    .collect();
                write!(f, "aggregate [{}]", formatted_values.join(", "))
            }
            Self::None => write!(f, "other (not a sync primitive)"),
        }
    }
}

impl std::fmt::Debug for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Single(single_value) => write!(f, "{single_value:?}"),
            Self::Aggregate(values) => {
                let formatted_values: Vec<String> =
                    values.iter().map(|value| format!("{value:?}")).collect();
                write!(f, "AGGREGATE [{}]", formatted_values.join(", "))
            }
            Self::None => write!(f, "OTHER (NOT A SYNC PRIMITIVE)"),
        }
    }
}

/// Possible single values that can be stored in the `Memory`
#[derive(Eq, PartialEq, Clone)]
pub enum Single {
    Mutex(MutexRef),
    MutexGuard(MutexGuardRef),
    JoinHandle(ThreadRef),
    Condvar(CondvarRef),
}

impl Single {
    pub const fn unpack_mutex(&self) -> Option<&MutexRef> {
        match self {
            Self::Mutex(ref mutex_ref) => Some(mutex_ref),
            _ => None,
        }
    }

    pub const fn unpack_mutex_guard(&self) -> Option<&MutexGuardRef> {
        match self {
            Self::MutexGuard(ref mutex_guard_ref) => Some(mutex_guard_ref),
            _ => None,
        }
    }

    pub const fn unpack_join_handle(&self) -> Option<&ThreadRef> {
        match self {
            Self::JoinHandle(ref thread_ref) => Some(thread_ref),
            _ => None,
        }
    }

    pub const fn unpack_condvar(&self) -> Option<&CondvarRef> {
        match self {
            Self::Condvar(ref condvar_ref) => Some(condvar_ref),
            _ => None,
        }
    }
}

impl std::fmt::Display for Single {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Mutex(_) => write!(f, "mutex"),
            Self::MutexGuard(_) => write!(f, "mutex guard"),
            Self::JoinHandle(_) => write!(f, "join handle"),
            Self::Condvar(_) => write!(f, "condition variable"),
        }
    }
}

impl std::fmt::Debug for Single {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Mutex(_) => write!(f, "MUTEX"),
            Self::MutexGuard(_) => write!(f, "MUTEX GUARD"),
            Self::JoinHandle(_) => write!(f, "JOIN HANDLE"),
            Self::Condvar(_) => write!(f, "CONDITION VARIABLE"),
        }
    }
}
