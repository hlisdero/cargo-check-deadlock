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
#[derive(PartialEq, Clone)]
pub enum Value {
    Mutex(MutexRef),
    MutexGuard(MutexGuardRef),
    JoinHandle(ThreadRef),
    Condvar(CondvarRef),
    Aggregate(Vec<Value>),
}

impl Value {
    pub fn unpack_mutex(&self) -> &MutexRef {
        match self {
            Self::Mutex(mutex_ref) => mutex_ref,
            _ => panic!("BUG: The value does not contain a mutex, it contains a {self}."),
        }
    }

    pub fn unpack_mutex_guard(&self) -> &MutexGuardRef {
        match self {
            Self::MutexGuard(mutex_guard_ref) => mutex_guard_ref,
            _ => panic!("BUG: The value does not contain a mutex guard, it contains a {self}."),
        }
    }

    pub fn unpack_join_handle(&self) -> &ThreadRef {
        match self {
            Self::JoinHandle(thread_ref) => thread_ref,
            _ => panic!("BUG: The value does not contain a join handle, it contains a {self}."),
        }
    }

    pub fn unpack_condvar(&self) -> &CondvarRef {
        match self {
            Self::Condvar(condvar_ref) => condvar_ref,
            _ => panic!(
                "BUG: The value does not contain a condition variable, it contains a {self}."
            ),
        }
    }

    pub fn unpack_aggregate(&self) -> &Vec<Self> {
        match self {
            Self::Aggregate(values) => values,
            _ => {
                panic!("BUG: The value does not contain an aggregate, it contains a {self}.")
            }
        }
    }
}

impl std::fmt::Display for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Mutex(_) => write!(f, "mutex"),
            Self::MutexGuard(_) => write!(f, "mutex guard"),
            Self::JoinHandle(_) => write!(f, "join handle"),
            Self::Condvar(_) => write!(f, "condition variable"),
            Self::Aggregate(_) => write!(f, "aggregate"),
        }
    }
}

impl std::fmt::Debug for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Mutex(_) => write!(f, "MUTEX"),
            Self::MutexGuard(_) => write!(f, "MUTEX GUARD"),
            Self::JoinHandle(_) => write!(f, "JOIN HANDLE"),
            Self::Condvar(_) => write!(f, "CONDITION VARIABLE"),
            Self::Aggregate(_) => write!(f, "AGGREGATE"),
        }
    }
}
