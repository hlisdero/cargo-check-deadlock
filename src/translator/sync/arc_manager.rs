//! Central structure to keep track of the `std::sync::Arc` in the code.
//!
//! It is mainly used in conjunction with the `MutexManager` to keep track of the mutexes
//! when they are wrapped around a `std::sync::Arc`.

use super::{
    detect_clone_arc_with_mutex, detect_deref_arc_with_mutex, detect_mutex_inside_arc_new,
};
use crate::naming::arc::function_transition_label;
use crate::translator::mir_function::Memory;
use crate::translator::special_function::call_foreign_function;
use netcrab::petri_net::{PetriNet, PlaceRef};

#[derive(Default)]
pub struct ArcManager {
    arc_counter: usize,
    deref_counter: usize,
    clone_counter: usize,
}

impl ArcManager {
    /// Returns a new empty `ArcManager`.
    pub fn new() -> Self {
        Self::default()
    }

    /// Translates a call to `std::sync::Arc::<T>::new` using
    /// the same representation as in `foreign_function_call`.
    /// A separate counter is incremented every time that
    /// the function is called to generate a unique label.
    pub fn translate_call_new(
        &mut self,
        start_place: &PlaceRef,
        end_place: &PlaceRef,
        net: &mut PetriNet,
    ) {
        let index = self.arc_counter;
        let transition_label = &function_transition_label("std::sync::Arc::<T>::new", index);
        self.arc_counter += 1;
        call_foreign_function(start_place, end_place, None, transition_label, net);
    }

    /// Translates a call to `std::ops::Deref::deref` using
    /// the same representation as in `foreign_function_call`.
    /// A separate counter is incremented every time that
    /// the function is called to generate a unique label.
    pub fn translate_call_deref(
        &mut self,
        start_place: &PlaceRef,
        end_place: &PlaceRef,
        cleanup_place: Option<PlaceRef>,
        net: &mut PetriNet,
    ) {
        let index = self.deref_counter;
        let transition_label = &function_transition_label("std::ops::Deref::deref", index);
        self.deref_counter += 1;
        call_foreign_function(start_place, end_place, cleanup_place, transition_label, net);
    }

    /// Translates a call to `std::clone::Clone::clone` using
    /// the same representation as in `foreign_function_call`.
    /// A separate counter is incremented every time that
    /// the function is called to generate a unique label.
    pub fn translate_call_clone(
        &mut self,
        start_place: &PlaceRef,
        end_place: &PlaceRef,
        cleanup_place: Option<PlaceRef>,
        net: &mut PetriNet,
    ) {
        let index = self.clone_counter;
        let transition_label = &function_transition_label("std::clone::Clone::clone", index);
        self.clone_counter += 1;
        call_foreign_function(start_place, end_place, cleanup_place, transition_label, net);
    }

    /// Translates the side effects for `std::sync::Arc::<T>::new` i.e.,
    /// detecting whether the return value should be linked to a mutex (because the `Arc` contains one).
    /// Receives a reference to the memory of the caller function to
    /// link the return local variable to the existing mutex.
    pub fn translate_side_effects_new(
        args: &[rustc_middle::mir::Operand],
        return_value: rustc_middle::mir::Place,
        body: &rustc_middle::mir::Body,
        memory: &mut Memory,
    ) {
        let first_argument = args
            .get(0)
            .expect("BUG: `std::sync::Arc::<T>::new` should receive at least one argument");

        if let Some((return_value_local, local_with_mutex)) =
            detect_mutex_inside_arc_new(first_argument, return_value, body)
        {
            memory.link_local_to_same_mutex(return_value_local, local_with_mutex);
        }
    }

    /// Translates the side effects for `std::ops::Deref::deref` i.e.,
    /// detecting whether the return value should be linked to a mutex (because the `Arc` contains one).
    /// Receives a reference to the memory of the caller function to
    /// link the return local variable to the existing mutex.
    pub fn translate_side_effects_deref(
        args: &[rustc_middle::mir::Operand],
        return_value: rustc_middle::mir::Place,
        body: &rustc_middle::mir::Body,
        memory: &mut Memory,
    ) {
        let first_argument = args
            .get(0)
            .expect("BUG: `std::ops::Deref::deref` should receive at least one argument");

        if let Some((return_value_local, local_with_mutex)) =
            detect_deref_arc_with_mutex(first_argument, return_value, body)
        {
            memory.link_local_to_same_mutex(return_value_local, local_with_mutex);
        }
    }

    /// Translates the side effects for `std::clone::Clone::clone` i.e.,
    /// detecting whether the return value should be linked to a mutex (because the `Arc` contains one).
    /// Receives a reference to the memory of the caller function to
    /// link the return local variable to the existing mutex.
    pub fn translate_side_effects_clone(
        args: &[rustc_middle::mir::Operand],
        return_value: rustc_middle::mir::Place,
        body: &rustc_middle::mir::Body,
        memory: &mut Memory,
    ) {
        let first_argument = args
            .get(0)
            .expect("BUG: `std::clone::Clone::clone` should receive at least one argument");

        if let Some((return_value_local, local_with_mutex)) =
            detect_clone_arc_with_mutex(first_argument, return_value, body)
        {
            memory.link_local_to_same_mutex(return_value_local, local_with_mutex);
        }
    }
}
