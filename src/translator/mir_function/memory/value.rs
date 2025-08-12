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
            Self::Single(_) => {
                panic!("BUG: The value does not contain a mutex, it contains: {self}.")
            }
            Self::Aggregate(values) => find_mutex_ref(values).map_or_else(
                || panic!("BUG: No mutex could be found in {self}."),
                |mutex_ref| mutex_ref,
            ),
        }
    }

    pub fn unpack_mutex_guard(&self) -> &MutexGuardRef {
        match self {
            Self::Single(Single::MutexGuard(mutex_guard_ref)) => mutex_guard_ref,
            Self::Single(_) => {
                panic!("BUG: The value does not contain a mutex guard, it contains: {self}.")
            }
            Self::Aggregate(values) => find_mutex_guard_ref(values).map_or_else(
                || panic!("BUG: No mutex guard could be found in {self}."),
                |mutex_guard_ref| mutex_guard_ref,
            ),
        }
    }

    pub fn unpack_join_handle(&self) -> &ThreadRef {
        match self {
            Self::Single(Single::JoinHandle(thread_ref)) => thread_ref,
            Self::Single(_) => {
                panic!("BUG: The value does not contain a join handle, it contains: {self}.")
            }
            Self::Aggregate(values) => find_join_handle_ref(values).map_or_else(
                || panic!("BUG: No join handle could be found in {self}."),
                |join_handle_ref| join_handle_ref,
            ),
        }
    }

    pub fn unpack_condvar(&self) -> &CondvarRef {
        match self {
            Self::Single(Single::Condvar(condvar_ref)) => condvar_ref,
            Self::Single(_) => {
                panic!("BUG: The value does not contain a condition variable, it contains: {self}.")
            }
            Self::Aggregate(values) => find_condvar_ref(values).map_or_else(
                || panic!("BUG: No condition variable could be found in {self}."),
                |condvar_ref| condvar_ref,
            ),
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
                let formatted_values: Vec<String> =
                    values.iter().map(|value| format!("{value:?}")).collect();
                write!(f, "AGGREGATE [{}]", formatted_values.join(", "))
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

pub fn find_mutex_ref(values: &Vec<Value>) -> Option<&MutexRef> {
    for value in values {
        match value {
            Value::Single(single) => {
                let result = single.unpack_mutex();
                if result.is_some() {
                    return result;
                }
            }
            Value::Aggregate(inner_values) => {
                if let Some(mutex_ref) = find_mutex_ref(inner_values) {
                    return Some(mutex_ref);
                }
            }
        }
    }
    None
}

pub fn find_mutex_guard_ref(values: &Vec<Value>) -> Option<&MutexGuardRef> {
    for value in values {
        match value {
            Value::Single(single) => {
                let result = single.unpack_mutex_guard();
                if result.is_some() {
                    return result;
                }
            }
            Value::Aggregate(inner_values) => {
                if let Some(mutex_guard_ref) = find_mutex_guard_ref(inner_values) {
                    return Some(mutex_guard_ref);
                }
            }
        }
    }
    None
}

pub fn find_join_handle_ref(values: &Vec<Value>) -> Option<&ThreadRef> {
    for value in values {
        match value {
            Value::Single(single) => {
                let result = single.unpack_join_handle();
                if result.is_some() {
                    return result;
                }
            }
            Value::Aggregate(inner_values) => {
                if let Some(join_handle_ref) = find_join_handle_ref(inner_values) {
                    return Some(join_handle_ref);
                }
            }
        }
    }
    None
}

pub fn find_condvar_ref(values: &Vec<Value>) -> Option<&CondvarRef> {
    for value in values {
        match value {
            Value::Single(single) => {
                let result = single.unpack_condvar();
                if result.is_some() {
                    return result;
                }
            }
            Value::Aggregate(inner_values) => {
                if let Some(condvar_ref) = find_condvar_ref(inner_values) {
                    return Some(condvar_ref);
                }
            }
        }
    }
    None
}
