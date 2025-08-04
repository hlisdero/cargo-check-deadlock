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
    Single(Single),
    Aggregate(Vec<Value>),
}

impl Value {
    pub fn unpack_mutex(&self) -> &MutexRef {
        match self {
            Self::Single(Single::Mutex(mutex_ref)) => mutex_ref,
            _ => panic!("BUG: The value does not contain a mutex, it contains: {self}."),
        }
    }

    pub fn unpack_mutex_guard(&self) -> &MutexGuardRef {
        match self {
            Self::Single(Single::MutexGuard(mutex_guard_ref)) => mutex_guard_ref,
            _ => panic!("BUG: The value does not contain a mutex guard, it contains: {self}."),
        }
    }

    pub fn unpack_join_handle(&self) -> &ThreadRef {
        match self {
            Self::Single(Single::JoinHandle(thread_ref)) => thread_ref,
            _ => panic!("BUG: The value does not contain a join handle, it contains: {self}."),
        }
    }

    pub fn unpack_condvar(&self) -> &CondvarRef {
        match self {
            Self::Single(Single::Condvar(condvar_ref)) => condvar_ref,
            _ => {
                panic!("BUG: The value does not contain a condition variable, it contains: {self}.")
            }
        }
    }

    pub fn unpack_aggregate(&self) -> &Vec<Self> {
        match self {
            Self::Single(_) => {
                panic!("BUG: The value does not contain an aggregate, it contains: {self}.")
            }
            Self::Aggregate(values) => values,
        }
    }
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
        }
    }
}

impl std::fmt::Debug for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Single(single_value) => write!(f, "{single_value:?}"),
            Self::Aggregate(values) => {
                let formatted_values: Vec<String> = values
                    .iter()
                    .map(std::string::ToString::to_string)
                    .collect();
                write!(f, "AGGREGATE [{:?}]", formatted_values.join(", "))
            }
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
    Other,
}

impl std::fmt::Display for Single {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Mutex(_) => write!(f, "mutex"),
            Self::MutexGuard(_) => write!(f, "mutex guard"),
            Self::JoinHandle(_) => write!(f, "join handle"),
            Self::Condvar(_) => write!(f, "condition variable"),
            Self::Other => write!(f, "other, not a sync primitive"),
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
            Self::Other => write!(f, "OTHER, NOT A SYNC PRIMITIVE"),
        }
    }
}
