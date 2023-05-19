//! Module that implements a mapping between
//! memory places (`rustc_middle::mir::Place`) and a given value.
//!
//! It is used to keep track of the sync variables
//! (mutexes, mutex guards, join handles and condition variables)
//! in every MIR function.
//!
//! The idea is to mark (link) a place
//! to a given sync variable when it is created.
//! When the sync variable gets assigned,
//! mark the new value as also containing the sync variable.
//! When passing the sync variable to a new thread,
//! find all places with the same local that contain a sync variable.

use log::debug;
use std::collections::HashMap;

type Place<'tcx> = rustc_middle::mir::Place<'tcx>;

pub struct MemoryMap<'tcx, T> {
    map: HashMap<Place<'tcx>, T>,
}

impl<'tcx, T> MemoryMap<'tcx, T>
where
    T: std::cmp::PartialEq<T> + std::clone::Clone,
{
    /// Creates a new `MemoryMap` with empty mappings.
    pub fn new() -> Self {
        Self {
            map: HashMap::new(),
        }
    }

    /// Links a given place to a given value.
    /// Prints debug messages if the place was already linked to a value.
    pub fn link_place(&mut self, place: Place<'tcx>, value: T) {
        if let Some(old_value) = self.map.insert(place, value) {
            let type_name = std::any::type_name::<T>();
            if old_value == self.map[&place] {
                debug!("PLACE {place:?} LINKED AGAIN TO SAME `{type_name}`",);
            } else {
                debug!("PLACE {place:?} LINKED TO A DIFFERENT `{type_name}`",);
            }
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
        self.link_place(place_to_link, std::clone::Clone::clone(value));
        let type_name = std::any::type_name::<T>();
        debug!("SAME `{type_name}`: {place_to_link:?} = {place_linked:?}",);
    }

    /// Returns a reference to the value linked to the given place.
    ///
    /// # Panics
    ///
    /// If the place is not linked to a value, then the function panics.
    pub fn get_linked_value(&self, place: &Place<'tcx>) -> &T {
        let _type_name = std::any::type_name::<T>();
        self.map
            .get(place)
            .expect("BUG: The place {place:?} should be linked to a `{_type_name}`")
    }

    /// Checks if the place is linked to a value.
    pub fn is_linked(&self, place: &Place<'tcx>) -> bool {
        self.map.contains_key(place)
    }

    /// Finds all the values linked to the given place.
    /// It takes into consideration that the place may have several fields (a subtype of projections).
    /// If the place given has the same local, then it is considered to be the same place.
    /// <https://rustc-dev-guide.rust-lang.org/mir/index.html?highlight=Projections#mir-data-types>
    /// Returns a vector of values.
    pub fn find_linked_values(&self, place: Place<'tcx>) -> Vec<T> {
        self.map
            .iter()
            .filter_map(|(k, v)| {
                if k.local == place.local {
                    Some(v.clone())
                } else {
                    None
                }
            })
            .collect()
    }
}
