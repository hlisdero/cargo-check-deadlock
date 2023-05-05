//! Central structure to keep track of indexed functions,
//! i.e., functions whose labels in the Petri net must be different for every call.
//!
//! A `HashMapCounter` keeps track of how many time each function name has been seen so far.
//! After every call the counter for the corresponding function is incremented.
//!
//! It is mainly used for functions that create synchronization variables
//! or functions whose Petri net representation should not be "reused" for clarity.

use crate::data_structures::hash_map_counter::HashMapCounter;
use crate::data_structures::petri_net_interface::{PetriNet, TransitionRef};
use crate::translator::function_call::FunctionPlaces;
use crate::translator::special_function::call_foreign_function;

#[derive(Default)]
pub struct FunctionCounter {
    counter: HashMapCounter,
}

impl FunctionCounter {
    /// Returns a new empty `FunctionCounter`.
    pub fn new() -> Self {
        Self {
            counter: HashMapCounter::new(),
        }
    }

    /// Translates a call to a function with given function name using
    /// the same representation as in `foreign_function_call`.
    ///
    /// Receives a labelling function that takes a function name and an index
    /// and returns two transition labels as expected by `foreign_function_call`.
    ///
    /// A separate counter is incremented every time that
    /// the function is called to generate a unique label.
    ///
    /// Returns the transition that represents the function call.
    pub fn translate_indexed_call(
        &mut self,
        function_name: &str,
        function_call_places: &FunctionPlaces,
        labelling_function: fn(&str, usize) -> (String, String),
        net: &mut PetriNet,
    ) -> TransitionRef {
        let index = self.counter.get_count(function_name);
        self.counter.increment(function_name);
        call_foreign_function(
            function_call_places,
            &labelling_function(function_name, index),
            net,
        )
    }

    /// Translates a call to a function with given function name using
    /// the same representation as in `foreign_function_call`.
    ///
    /// Receives a labelling function that takes an index and returns two
    /// transition labels as expected by `foreign_function_call`.
    ///
    /// A separate counter is incremented every time that
    /// the function is called to generate a unique label.
    ///
    /// Returns the transition that represents the function call.
    pub fn translate_call(
        &mut self,
        function_name: &str,
        function_call_places: &FunctionPlaces,
        labelling_function: fn(usize) -> (String, String),
        net: &mut PetriNet,
    ) -> TransitionRef {
        let index = self.counter.get_count(function_name);
        self.counter.increment(function_name);
        call_foreign_function(function_call_places, &labelling_function(index), net)
    }
}
