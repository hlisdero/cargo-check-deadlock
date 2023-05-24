//! Representation of the local memory of the function.
//!
//! It implements a mapping between
//! memory places (`rustc_middle::mir::Place`) and a variant of `Value`.
//!
//! It is used to keep track of the sync variables
//! (mutexes, mutex guards, join handles and condition variables)
//! in every MIR function.
//!
//! The idea is to mark (link) a place
//! to a given sync variable when it is created.
//! When the sync variable gets assigned,
//! mark the new value as also containing the sync variable.
//! When the sync variables are packed into a tuple or an `std::sync::Arc`,
//! create an aggregate and pass it to new threads.
//! The aggregated value can be accessed
//! and its fields mapped to the memory of the new function.
//!
//! More info:
//! <https://rustc-dev-guide.rust-lang.org/mir/index.html#mir-data-types>

use log::debug;
use std::collections::HashMap;

use crate::translator::sync::{CondvarRef, MutexGuardRef, MutexRef, ThreadRef};

use Value::{Aggregate, Condvar, JoinHandle, Mutex, MutexGuard};
type Place<'tcx> = rustc_middle::mir::Place<'tcx>;

/// Print a debug message about a place that was linked to the same value twice.
macro_rules! debug_same_type_same_value {
    ($place:expr, $old_value:expr) => {
        debug!("PLACE {:?} LINKED AGAIN TO SAME {:?}", $place, $old_value);
    };
}

/// Print a debug message about a place that was linked to a different value of the same type.
macro_rules! debug_same_type_different_value {
    ($place:expr, $old_value:expr) => {
        debug!(
            "PLACE {:?} WAS LINKED TO A DIFFERENT {:?}",
            $place, $old_value
        );
    };
}

/// Print a debug message about a place that was linked to a value of a different type.
macro_rules! debug_different_type {
    ($place:expr, $old_value:expr) => {
        debug!("PLACE {:?} WAS LINKED TO A {:?}", $place, $old_value);
    };
}

#[derive(Default)]
pub struct Memory<'tcx> {
    map: HashMap<Place<'tcx>, Value>,
}

impl<'tcx> Memory<'tcx> {
    /// Links a given place to a given mutex.
    /// Prints debug messages if the place was already linked.
    pub fn link_mutex(&mut self, place: Place<'tcx>, mutex_ref: &MutexRef) {
        if let Some(old_value) = self.map.insert(place, Mutex(mutex_ref.clone())) {
            let type_string = old_value.to_string();

            if let Mutex(old_mutex_ref) = old_value {
                if *mutex_ref == old_mutex_ref {
                    debug_same_type_same_value!(place, type_string);
                } else {
                    debug_same_type_different_value!(place, type_string);
                }
            } else {
                debug_different_type!(place, type_string);
            }
        }
    }

    /// Links a given place to a given mutex guard.
    /// Prints debug messages if the place was already linked.
    pub fn link_mutex_guard(&mut self, place: Place<'tcx>, mutex_guard_ref: &MutexGuardRef) {
        if let Some(old_value) = self.map.insert(place, MutexGuard(mutex_guard_ref.clone())) {
            let type_string = old_value.to_string();

            if let MutexGuard(old_mutex_guard_ref) = old_value {
                if *mutex_guard_ref == old_mutex_guard_ref {
                    debug_same_type_same_value!(place, type_string);
                } else {
                    debug_same_type_different_value!(place, type_string);
                }
            } else {
                debug_different_type!(place, type_string);
            }
        }
    }

    /// Links a given place to a given join handle.
    /// Prints debug messages if the place was already linked.
    pub fn link_join_handle(&mut self, place: Place<'tcx>, thread_ref: &ThreadRef) {
        if let Some(old_value) = self.map.insert(place, JoinHandle(thread_ref.clone())) {
            let type_string = old_value.to_string();

            if let JoinHandle(old_thread_ref) = old_value {
                if *thread_ref == old_thread_ref {
                    debug_same_type_same_value!(place, type_string);
                } else {
                    debug_same_type_different_value!(place, type_string);
                }
            } else {
                debug_different_type!(place, type_string);
            }
        }
    }

    /// Links a given place to a given condition variable.
    /// Prints debug messages if the place was already linked.
    pub fn link_condvar(&mut self, place: Place<'tcx>, condvar_ref: &CondvarRef) {
        if let Some(old_value) = self.map.insert(place, Condvar(condvar_ref.clone())) {
            let type_string = old_value.to_string();

            if let Condvar(old_condvar_ref) = old_value {
                if *condvar_ref == old_condvar_ref {
                    debug_same_type_same_value!(place, type_string);
                } else {
                    debug_same_type_different_value!(place, type_string);
                }
            } else {
                debug_different_type!(place, type_string);
            }
        }
    }

    /// Links a given place to a given aggregate.
    ///
    /// # Panics
    ///
    /// If the place was already linked, then the function panics.
    pub fn link_aggregate(&mut self, place: Place<'tcx>, values: Vec<Value>) {
        if let Some(old_value) = self.map.insert(place, Aggregate(values)) {
            panic!(
                "BUG: There was a previous {old_value} linked to the place for the aggregate value"
            );
        }
    }

    /// Links two places to the same value.
    /// After this operation, both places point to the same value, i.e.
    /// the first place is an alias for the second place.
    ///
    /// # Panics
    ///
    /// If the `place_linked` is not linked to a value, then the function panics.
    pub fn link_place_to_same_value(
        &mut self,
        place_to_link: Place<'tcx>,
        place_linked: Place<'tcx>,
    ) {
        let value = self.map.get(&place_linked).unwrap_or_else(|| {
            panic!("BUG: The place {place_linked:?} should be linked to a value")
        });
        let cloned_value = std::clone::Clone::clone(value);

        if let Some(old_value) = self.map.insert(place_to_link, value.clone()) {
            if old_value == cloned_value {
                debug_same_type_different_value!(place_to_link, old_value);
            } else {
                debug_different_type!(place_to_link, old_value);
            }
        }
        debug!("SAME VALUE: {place_to_link:?} = {place_linked:?}",);
    }

    /// Returns an immutable reference to the value linked to the given place.
    ///
    /// # Panics
    ///
    /// If the place is not linked to a value, then the function panics.
    fn get_linked_value(&self, place: &Place<'tcx>) -> &Value {
        self.map
            .get(place)
            .unwrap_or_else(|| panic!("BUG: The place {place:?} should be linked to a value"))
    }

    /// Returns a reference to the mutex linked to the given place.
    pub fn get_mutex(&self, place: &Place<'tcx>) -> &MutexRef {
        self.get_linked_value(place).unpack_mutex()
    }

    /// Returns a reference to the mutex guard linked to the given place.
    pub fn get_mutex_guard(&self, place: &Place<'tcx>) -> &MutexGuardRef {
        self.get_linked_value(place).unpack_mutex_guard()
    }

    /// Returns a reference to the join handle linked to the given place.
    pub fn get_join_handle(&self, place: &Place<'tcx>) -> &ThreadRef {
        self.get_linked_value(place).unpack_join_handle()
    }

    /// Returns a reference to the condition variable linked to the given place.
    pub fn get_condvar(&self, place: &Place<'tcx>) -> &CondvarRef {
        self.get_linked_value(place).unpack_condvar()
    }

    /// Returns the vector of values contained inside the aggregate linked to the given place.
    /// The vector is copied for the caller since the value may be used later by this function.
    ///
    /// # Panics
    ///
    /// If the place is not linked to a value, then the function panics.
    pub fn copy_aggregate(&mut self, place: &Place<'tcx>) -> Vec<Value> {
        self.map
            .get(place)
            .unwrap_or_else(|| panic!("BUG: The place {place:?} should be linked to an aggregate"))
            .unpack_aggregate()
            .clone()
    }

    /// Checks whether the place is linked to a mutex guard.
    pub fn is_mutex_guard(&self, place: &Place<'tcx>) -> bool {
        self.map.contains_key(place) && matches!(self.get_linked_value(place), MutexGuard(_))
    }

    /// Creates a new aggregate value from the values linked to a vector of places.
    /// Links the new aggregate value to the given place.
    ///
    /// # Panics
    ///
    /// If there is a value linked to the place for the aggregate, then the function panics.
    pub fn create_aggregate(&mut self, place: Place<'tcx>, places_to_aggregate: &[Place<'tcx>]) {
        let mut values: Vec<Value> = Vec::new();

        for place in places_to_aggregate {
            let value = self.get_linked_value(place);
            values.push(value.clone());
        }

        self.link_aggregate(place, values);
    }

    /// Links the field of an aggregate to a given place.
    /// This is equivalent to moving out the value from the aggregate
    /// and linking it to the place.
    ///
    /// # Panics
    ///
    /// If the place if not linked to an aggregate, then the function panics.
    /// If the index is out of bounds, then the function panics.
    pub fn link_field_in_aggregate(
        &mut self,
        place_to_link: Place<'tcx>,
        place_linked: Place<'tcx>,
        index: usize,
    ) {
        let values = self
            .map
            .get_mut(&place_linked)
            .unwrap_or_else(|| {
                panic!("BUG: The place {place_linked:?} should be linked to a value")
            })
            .unpack_aggregate();
        let value = values
            .get(index)
            .unwrap_or_else(|| {
                panic!(
                    "BUG: The place {place_linked:?} does not contain a field with index {index}"
                )
            })
            .clone();

        match value {
            Value::Mutex(mutex_ref) => self.link_mutex(place_to_link, &mutex_ref),
            Value::MutexGuard(mutex_guard_ref) => {
                self.link_mutex_guard(place_to_link, &mutex_guard_ref);
            }
            Value::JoinHandle(thread_ref) => self.link_join_handle(place_to_link, &thread_ref),
            Value::Condvar(condvar_ref) => self.link_condvar(place_to_link, &condvar_ref),
            Value::Aggregate(values) => self.link_aggregate(place_to_link, values),
        }
    }
}

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
    fn unpack_mutex(&self) -> &MutexRef {
        match self {
            Mutex(mutex_ref) => mutex_ref,
            _ => panic!("BUG: The value does not contain a mutex, it contains a {self}."),
        }
    }

    fn unpack_mutex_guard(&self) -> &MutexGuardRef {
        match self {
            MutexGuard(mutex_guard_ref) => mutex_guard_ref,
            _ => panic!("BUG: The value does not contain a mutex guard, it contains a {self}."),
        }
    }

    fn unpack_join_handle(&self) -> &ThreadRef {
        match self {
            JoinHandle(thread_ref) => thread_ref,
            _ => panic!("BUG: The value does not contain a join handle, it contains a {self}."),
        }
    }

    fn unpack_condvar(&self) -> &CondvarRef {
        match self {
            Condvar(condvar_ref) => condvar_ref,
            _ => panic!(
                "BUG: The value does not contain a condition variable, it contains a {self}."
            ),
        }
    }

    fn unpack_aggregate(&self) -> &Vec<Self> {
        match self {
            Aggregate(values) => values,
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
