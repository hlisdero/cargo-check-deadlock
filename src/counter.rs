//! Module that implements a simple hash map with counters.
//!
//! It is used to keep track of how many times a function
//! (represented by its function name as a `String`) was seen so far
//! during the translation process.

use std::collections::HashMap;

#[derive(Default)]
pub struct Counter {
    counts: HashMap<String, usize>,
}

impl Counter {
    /// Creates a new empty counter.
    pub fn new() -> Self {
        Self::default()
    }

    /// Increments the counter by one.
    /// Adds it to the counter if it does not exist yet and
    /// initializes the count to one.
    pub fn increment(&mut self, key: String) {
        if let Some(count) = self.counts.get(&key) {
            self.counts.insert(key, count + 1);
        } else {
            self.counts.insert(key, 1);
        }
    }

    /// Returns the count for a given key.
    pub fn get_count(&mut self, key: &str) -> usize {
        if let Some(count) = self.counts.get(key) {
            *count
        } else {
            self.counts.insert(key.to_string(), 0);
            0
        }
    }
}

#[cfg(test)]
mod counter_tests {
    use super::*;

    #[test]
    fn counter_new_is_empty() {
        let counter: Counter = Counter::new();

        assert!(counter.counts.is_empty());
    }

    #[test]
    fn counter_get_count_of_new_key_returns_zero() {
        let mut counter: Counter = Counter::new();

        assert_eq!(counter.get_count("Example key"), 0);
    }

    #[test]
    fn counter_increment_updates_count() {
        let mut counter: Counter = Counter::new();
        counter.increment("Example key".to_string());

        assert_eq!(counter.get_count("Example key"), 1);
    }

    #[test]
    fn counter_increment_many_times() {
        let mut counter: Counter = Counter::new();

        for i in 0..10 {
            counter.increment("Example key".to_string());
        }

        assert_eq!(counter.get_count("Example key"), 10);
    }
}
