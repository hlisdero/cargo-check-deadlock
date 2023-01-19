use std::collections::HashMap;

#[derive(Default)]
pub struct Counter {
    counts: HashMap<String, usize>,
}

impl Counter {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn increment(&mut self, key: String) {
        if let Some(count) = self.counts.get(&key) {
            self.counts.insert(key, count + 1);
        } else {
            self.counts.insert(key, 0);
        }
    }

    pub fn get_count(&mut self, key: &str) -> usize {
        if let Some(count) = self.counts.get(key) {
            *count
        } else {
            self.counts.insert(key.to_string(), 0);
            0
        }
    }
}
