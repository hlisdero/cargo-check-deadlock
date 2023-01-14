//! Auxiliary structure to keep track of the mutexes in the code.
//!
//! The `MutexManager` stores the mutexes discovered so far in the code.
//! It also stores the lock guards, i.e. the local variables that result
//! from a call to `std::sync::Mutex::lock()`.
//! It also stores the links, i.e. the local variables that reference a mutex.
//! If a mutex A is accessed in two threads then two variables
//! (the original and the `.clone()` of the `std::sync::Arc`) is a link to mutex A.

use crate::translator::naming::mutex_function_transition_label;
use crate::translator::special_function::foreign_function_call;
use crate::translator::sync::mutex::Mutex;
use crate::translator::utils::place_to_local;
use netcrab::petri_net::{PetriNet, PlaceRef, TransitionRef};
use std::collections::HashMap;

#[derive(Default)]
pub struct MutexManager {
    mutexes: Vec<Mutex>,
    locals_linked_to_mutexes: HashMap<rustc_middle::mir::Local, MutexRef>,
    lock_counter: usize,
}

/// A wrapper type around the indexes to the elements in `Vec<Mutex>`.
#[derive(Clone)]
struct MutexRef(usize);

impl MutexManager {
    /// Returns a new empty `MutexManager`
    pub fn new() -> Self {
        Self::default()
    }

    /// Translates a mutex function call using the same logic as in `foreign_function_call`.
    /// Returns the transition that represents the function call.
    ///
    /// The naming depends:
    /// - For `std::sync::Mutex::<T>::new` the numbering follows the numbering of the labels of the mutexes.
    /// - For `std::sync::Mutex::<T>::lock` a separate counter is incremented every time that the function is called.
    pub fn translate_function_call(
        &self,
        function_name: &str,
        start_place: &PlaceRef,
        end_place: &PlaceRef,
        cleanup_place: Option<PlaceRef>,
        net: &mut PetriNet,
    ) -> TransitionRef {
        let index = if function_name == "std::sync::Mutex::<T>::new" {
            self.mutexes.len()
        } else {
            self.lock_counter
        };

        let transition_label = &mutex_function_transition_label(function_name, index);
        foreign_function_call(start_place, end_place, cleanup_place, transition_label, net)
    }

    /// Translates the side effects for the methods of `std::sync::Mutex`, i.e.,
    /// the specific logic of creating a new mutex and locking it.
    ///
    /// # Panics
    ///
    /// If the function name is not one of the supported mutex functions, then the function panics.
    pub fn translate_function_side_effects(
        &mut self,
        function_name: &str,
        args: &[rustc_middle::mir::Operand],
        return_value: rustc_middle::mir::Place,
        transition_function_call: &TransitionRef,
        net: &mut PetriNet,
    ) {
        match function_name {
            "std::sync::Mutex::<T>::new" => {
                self.add_mutex(place_to_local(&return_value), net);
            }
            "std::sync::Mutex::<T>::lock" => {
                let mutex_to_lock =
                    Self::extract_self_reference_from_arguments_to_function_call(args);
                self.add_lock_guard(
                    place_to_local(&mutex_to_lock),
                    transition_function_call,
                    net,
                );
                self.lock_counter += 1;
            }
            _ => unimplemented!("Mutex method {function_name} not implemented yet"),
        }
    }

    /// Adds a new mutex and creates its corresponding representation in the Petri net.
    /// Links the given local, the return value of `std::sync::Mutex<T>::new()`, to the new mutex.
    ///
    /// # Panics
    ///
    /// If the `return_local` is already linked to a mutex, then the function panics.
    fn add_mutex(&mut self, return_local: rustc_middle::mir::Local, net: &mut PetriNet) {
        let index = self.mutexes.len();
        if self
            .locals_linked_to_mutexes
            .insert(return_local, MutexRef(index))
            .is_some()
        {
            panic!("BUG: The return value of `std::sync::Mutex<T>::new()` should not be already linked to a mutex")
        }
        self.mutexes.push(Mutex::new(index, net));
    }

    /// Adds a lock guard to the mutex.
    /// Connects the mutex's place to the transition, then the transition will only
    /// fire if the mutex is unlocked.
    ///
    /// # Panics
    ///
    /// If the local variable is not linked to a mutex, then the function panics.
    fn add_lock_guard(
        &mut self,
        local_with_mutex: rustc_middle::mir::Local,
        transition_lock: &TransitionRef,
        net: &mut PetriNet,
    ) {
        let mutex = self.get_mutex_from_local(local_with_mutex);
        mutex.add_lock_guard(transition_lock, net);
    }

    /// Links a local variable to the mutex linked to another local.
    /// After this operation both locals point to the same mutex, i.e.,
    /// the first local is an alias for the second local.
    ///
    /// # Panics
    ///
    /// If the local to be linked is already linked to a mutex, then the function panics.
    /// If the local linked to a mutex is not linked to a mutex, then the function panics.
    pub fn link_local_to_same_mutex(
        &mut self,
        local_to_be_linked: rustc_middle::mir::Local,
        local_linked_to_mutex: rustc_middle::mir::Local,
    ) {
        let mutex_ref = self
            .locals_linked_to_mutexes
            .get(&local_linked_to_mutex)
            .expect("BUG: The local variable should be linked to a mutex");
        if self
            .locals_linked_to_mutexes
            .insert(local_to_be_linked, mutex_ref.clone())
            .is_some()
        {
            panic!("BUG: The local to be linked should not be linked to another mutex")
        }
    }

    /// Returns the mutex contained in the `local` variable.
    ///
    /// # Panics
    ///
    /// If the `local` is not associated to a mutex, then the function panics.
    fn get_mutex_from_local(&self, local: rustc_middle::mir::Local) -> &Mutex {
        let mutex_ref = self
            .locals_linked_to_mutexes
            .get(&local)
            .expect("BUG: The local variable should be linked to a mutex");
        self.mutexes
            .get(mutex_ref.0)
            .expect("BUG: The mutex reference stored for the local variable should be a valid index for the array of mutexes")
    }

    /// Extracts the self reference from the function arguments.
    /// For example: The call `mutex.lock()` desugars to `std::sync::Mutex::lock(&mutex)`
    /// where `&self` is the first argument.
    fn extract_self_reference_from_arguments_to_function_call<'tcx>(
        args: &[rustc_middle::mir::Operand<'tcx>],
    ) -> rustc_middle::mir::Place<'tcx> {
        let rustc_middle::mir::Operand::Move(mutex_to_lock) = args.get(0)
            .expect("BUG: Mutex function should receive a reference to the mutex as the 0-th function argument") else { 
                panic!("BUG: The self reference to the mutex should be passed by moving");
        };
        *mutex_to_lock
    }
}
