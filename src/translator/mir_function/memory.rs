//! Representation of the local memory of the function.
//!
//! It implements a mapping between locals and a variant of `Value`.
//! For ease of use, it receives memory places (`rustc_middle::mir::Place`)
//! which it can unpack transparently and deal with fields accordingly.
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
//! then the value is of variant `Aggregate`.
//! Aggregates support containing other aggregates inside (nesting).
//!
//! More info:
//! <https://rustc-dev-guide.rust-lang.org/mir/index.html#mir-data-types>

mod mir_locals_vec;
mod value;

use log::debug;
use mir_locals_vec::MirLocalsVec;
use std::rc::Rc;

use crate::translator::sync::{Condvar, Mutex, MutexGuard, Thread};
pub use value::{CondvarRef, MutexGuardRef, MutexRef, Single, ThreadRef, Value};

type Local = usize;
type FieldNumber = usize;

type Place<'tcx> = rustc_middle::mir::Place<'tcx>;

#[derive(Default)]
pub struct Memory {
    data: MirLocalsVec,
}

impl std::fmt::Debug for Memory {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "{:?}", self.data)
    }
}

impl<'tcx> Memory {
    /// Converts the place into a `Local` and a vector of `FieldNumber`.
    /// Ignores the other variants of projection elements.
    fn extract_local_and_field_number(place: &Place<'tcx>) -> (Local, Vec<FieldNumber>) {
        let field_numbers: Vec<usize> = place
            .projection
            .iter()
            .filter_map(|projection_elem| {
                if let rustc_middle::mir::ProjectionElem::Field(number, _) = projection_elem {
                    debug!(
                        "PLACE {place:?} HAS A FIELD WITH NUMBER {:?}",
                        number.as_usize()
                    );
                    Some(number.as_usize())
                } else {
                    None
                }
            })
            .collect();

        (place.local.into(), field_numbers)
    }

    /// Links any `Value` to the `Local` contained in the place.
    /// Supports field numbers but only one level.
    /// Returns a reference to the linked value.
    ///
    /// # Panics
    ///
    /// If the place contains more than 2 levels of field indirection, then the function panics
    pub fn link(&mut self, place: Place<'tcx>, value: Value) -> &Value {
        let (local, field_numbers) = Self::extract_local_and_field_number(&place);

        match &mut self.data[local] {
            Value::None => {
                self.data[local] = value;
                &self.data[local]
            }
            Value::Single(_) => {
                if self.data[local] == value {
                    // In some cases the MIR shows two separate assignments that
                    // lead to the same linking. Nothing to do in this case.
                    debug!("PLACE {place:?} LINKED TO SAME {:?}", self.data[local]);
                    &self.data[local]
                } else {
                    // The type of the local never changes, this is a hard error.
                    panic!("PLACE {place:?} WAS LINKED TO A {:?}", self.data[local]);
                }
            }
            Value::Aggregate(old_values) => {
                assert!(
                    !field_numbers.is_empty(),
                    "BUG: {place:?} was linked to an Aggregate {old_values:?}"
                );
                assert!(
                    field_numbers.len() <= 1,
                    "BUG: Value can only be linked with one level of field indirection, found {}",
                    field_numbers.len()
                );
                old_values[field_numbers[0]] = value;
                &self.data[local]
            }
        }
    }

    /// Links a given place to a given mutex.
    /// Prints debug messages if the place was already linked.
    /// Returns a reference to the linked mutex.
    pub fn link_mutex(&mut self, place: Place<'tcx>, mutex: Mutex) -> &MutexRef {
        let mutex_ref: Rc<Mutex> = Rc::new(mutex);
        let value = Value::Single(Single::Mutex(mutex_ref));
        match self.link(place, value) {
            Value::Single(Single::Mutex(mutex_ref)) => mutex_ref,
            value => panic!("BUG: Stored a mutex but got {value:?} back"),
        }
    }

    /// Links a given place to a given mutex guard.
    /// Prints debug messages if the place was already linked.
    /// Returns a reference to the linked mutex guard.
    pub fn link_mutex_guard(
        &mut self,
        place: Place<'tcx>,
        mutex_guard: MutexGuard,
    ) -> &MutexGuardRef {
        let mutex_guard_ref = Rc::new(mutex_guard);
        let value = Value::Single(Single::MutexGuard(mutex_guard_ref));
        match self.link(place, value) {
            Value::Single(Single::MutexGuard(mutex_guard_ref)) => mutex_guard_ref,
            value => panic!("BUG: Stored a mutex guard but got {value:?} back"),
        }
    }

    /// Links a given place to a given join handle.
    /// Prints debug messages if the place was already linked.
    /// Returns a reference to the linked join handle.
    pub fn link_join_handle(&mut self, place: Place<'tcx>, thread: Thread) -> &ThreadRef {
        let thread_ref = Rc::new(thread);
        let value = Value::Single(Single::JoinHandle(thread_ref));
        match self.link(place, value) {
            Value::Single(Single::JoinHandle(thread_ref)) => thread_ref,
            value => panic!("BUG: Stored a join handle but got {value:?} back"),
        }
    }

    /// Links a given place to a given condition variable.
    /// Prints debug messages if the place was already linked.
    /// Returns a reference to the linked condition variable.
    pub fn link_condvar(&mut self, place: Place<'tcx>, condvar: Condvar) -> &CondvarRef {
        let condvar_ref = Rc::new(condvar);
        let value = Value::Single(Single::Condvar(condvar_ref));
        match self.link(place, value) {
            Value::Single(Single::Condvar(condvar_ref)) => condvar_ref,
            value => panic!("BUG: Stored a condition variable but got {value:?} back"),
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
        let value = self.get_linked_value(&place_linked);
        self.link(place_to_link, value.clone());
        debug!("SAME VALUE: {place_to_link:?} = {place_linked:?}",);
    }

    // Checks if the place is linked to a value, i.e, it contains a sync variable
    // or an aggregate that contains a sync variable.
    pub fn has_linked_value(&self, place: &Place<'tcx>) -> bool {
        let (local, field_numbers) = Self::extract_local_and_field_number(place);
        if local >= self.data.len() {
            return false;
        }

        match &self.data[local] {
            Value::None => false,
            Value::Single(_) => true,
            Value::Aggregate(values) => Self::find_value(values, &field_numbers),
        }
    }

    /// Looks for a `Single` value stored in the local and the field numbers.
    /// Supports any level of field indirection, e.g. `_2 = _1.0.1.2.3`
    /// If it finds it, returns true. Otherwise false.
    fn find_value(values: &[Value], field_numbers: &[FieldNumber]) -> bool {
        if field_numbers.is_empty() {
            return true; // An aggregate was found, nothing more to do
        }
        let mut index = 0;
        let last_index = field_numbers.len() - 1;
        let mut current_values = values;

        while index <= last_index {
            let next_index = field_numbers[index];

            match current_values.get(next_index) {
                None => return false,
                Some(Value::Single(_) | Value::None) => {
                    if index != last_index {
                        return false; // Reached a single value but there are more field indexes pending
                    }
                    return true;
                }
                Some(Value::Aggregate(next_values)) => {
                    if index == last_index {
                        return true;
                    }
                    current_values = next_values;
                    index += 1;
                }
            }
        }
        false
    }

    /// Returns an immutable reference to the value linked to the given place.
    /// If the place contains fields, it accesses the aggregates until it finds the value.
    ///
    /// # Panics
    ///
    /// If the place is not linked to a value, then the function panics.
    pub fn get_linked_value(&self, place: &Place<'tcx>) -> &Value {
        let (local, field_numbers) = Self::extract_local_and_field_number(place);
        assert!(
            local < self.data.len(),
            "BUG: The place {place:?} is out of bound with respect to the memory data"
        );

        match &self.data[local] {
            Value::None => {
                panic!("BUG: The place {place:?} should be linked to a value")
            }
            value @ Value::Aggregate(values) => {
                if field_numbers.is_empty() {
                    value
                } else {
                    Self::get_value(values, &field_numbers)
                }
            }
            value @ Value::Single(_) => value,
        }
    }

    /// Get to the `Single` value stored in the local and the field numbers.
    /// Supports any level of field indirection, e.g. `_2 = _1.0.1.2.3`
    ///
    /// # Panics
    ///
    /// If a single value cannot be found, then the function panics.
    fn get_value<'value>(values: &'value [Value], field_numbers: &[FieldNumber]) -> &'value Value {
        let mut index = 0;
        let last_index = field_numbers.len() - 1;
        let mut current_values = values;

        while index <= last_index {
            let next_index = field_numbers[index];

            match current_values.get(next_index) {
                None => panic!("BUG: A single value in {current_values:?} with field number {next_index} cannot be found"),
                Some(value) => {
                    if index == last_index {
                        return value;
                    }
                    match value {
                        Value::Single(_) | Value::None => panic!("BUG: Encountered a single value where an aggregate was expected"),
                        Value::Aggregate(next_values) => {
                            current_values = next_values;
                            index += 1;
                        }
                    }
                },
            }
        }
        panic!("BUG: A single value could not be found after traversing all field numbers");
    }

    /// Returns the return value (PLACE `_0`) if the place is linked to a value, otherwise returns `None`.
    pub fn get_return_value(&self) -> Option<Value> {
        let return_place = rustc_middle::mir::Place {
            local: rustc_middle::mir::Local::from_usize(0),
            projection: rustc_middle::ty::List::empty(),
        };

        if self.has_linked_value(&return_place) {
            Some(self.get_linked_value(&return_place).clone())
        } else {
            None
        }
    }

    /// Returns a copy of the value linked to the given place.
    /// If the place contains fields, it accesses the aggregates until it finds the value.
    /// If the place is not linked, it returns `None`.
    pub fn get_linked_value_or_none(&self, place: &Place<'tcx>) -> Option<Value> {
        if self.has_linked_value(place) {
            Some(self.get_linked_value(place).clone())
        } else {
            None
        }
    }

    /// Returns a reference to the mutex linked to the given place.
    pub fn get_mutex(&self, place: &Place<'tcx>) -> &MutexRef {
        match self.get_linked_value(place) {
            Value::Single(single) => single.unpack_mutex().unwrap(),
            value @ (Value::Aggregate(_) | Value::None) => {
                panic!("BUG: The value does not contain a mutex, it contains: {value:?}.")
            }
        }
    }

    /// Returns a reference to the mutex guard linked to the given place.
    pub fn get_mutex_guard(&self, place: &Place<'tcx>) -> &MutexGuardRef {
        match self.get_linked_value(place) {
            Value::Single(single) => single.unpack_mutex_guard().unwrap(),
            value @ (Value::Aggregate(_) | Value::None) => {
                panic!("BUG: The value does not contain a mutex guard, it contains: {value:?}.")
            }
        }
    }

    /// Returns a reference to the join handle linked to the given place.
    pub fn get_join_handle(&self, place: &Place<'tcx>) -> &ThreadRef {
        match self.get_linked_value(place) {
            Value::Single(single) => single.unpack_join_handle().unwrap(),
            value @ (Value::Aggregate(_) | Value::None) => {
                panic!("BUG: The value does not contain a join handle, it contains: {value:?}.")
            }
        }
    }

    /// Returns a reference to the condition variable linked to the given place.
    pub fn get_condvar(&self, place: &Place<'tcx>) -> &CondvarRef {
        match self.get_linked_value(place) {
            Value::Single(single) => single.unpack_condvar().unwrap(),
            value @ (Value::Aggregate(_) | Value::None) => panic!(
                "BUG: The value does not contain a condition variable, it contains: {value:?}."
            ),
        }
    }

    /// Checks whether the place is linked to a mutex guard.
    pub fn is_mutex_guard(&self, place: &Place<'tcx>) -> bool {
        if !self.has_linked_value(place) {
            return false;
        }
        let value = self.get_linked_value(place);
        matches!(value, Value::Single(Single::MutexGuard(_)))
    }

    /// Creates a new aggregate value from the places with sync variables to aggregate.
    /// It maps `Some(place)` to the corresponding linked value.
    /// It maps `None` to a `Other` type of value.
    /// Links the new aggregate value to the given place.
    ///
    /// # Panics
    ///
    /// If there is a value linked to the place for the aggregate, then the function panics.
    pub fn create_aggregate(
        &mut self,
        place: Place<'tcx>,
        places_of_aggregate_fields: &[Option<Place<'tcx>>],
    ) {
        let values: Vec<Value> = places_of_aggregate_fields
            .iter()
            .map(|place| {
                place.as_ref().map_or(Value::None, |place| {
                    let value = self.get_linked_value(place);
                    value.clone()
                })
            })
            .collect();

        self.link(place, Value::Aggregate(values.clone()));
        debug!("CREATED AGGREGATE AT {place:?} WITH VALUES {values:?}");
    }
}
