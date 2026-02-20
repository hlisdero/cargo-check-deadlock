//! Module that implements a simple wrapper around a `Vec`
//! for storing MIR locals mapped to `Value`.

use std::fmt;
use std::ops::{Index, IndexMut};

use super::value::Value;

/// A vector-like structure for storing MIR locals mapped to `Value`.
/// It automatically resizes on out-of-bound indexing and fills with `Value::None`.
#[derive(Clone)]
pub struct MirLocalsVec {
    data: Vec<Value>,
}

impl Default for MirLocalsVec {
    fn default() -> Self {
        Self {
            data: vec![Value::None; Self::INITIAL_SIZE],
        }
    }
}

impl MirLocalsVec {
    const INITIAL_SIZE: usize = 10;

    /// Returns the number of elements in the vector, also referred to as its 'length'.
    pub const fn len(&self) -> usize {
        self.data.len()
    }

    /// Ensure that the vector has at least `index` elements.
    /// Use a growth factor of 1.5 since 2 seemed too big. This could be tweaked if needed.
    fn ensure_capacity(&mut self, index: usize) {
        if index >= self.data.len() {
            let new_len = index + index / 2;
            self.data.resize(new_len, Value::None);
        }
    }
}

impl Index<usize> for MirLocalsVec {
    type Output = Value;

    fn index(&self, index: usize) -> &Self::Output {
        &self.data[index]
    }
}

impl IndexMut<usize> for MirLocalsVec {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        self.ensure_capacity(index);
        &mut self.data[index]
    }
}

impl fmt::Debug for MirLocalsVec {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // Write the header for the table
        writeln!(f, "\n")?; // New line for better logs
        writeln!(f, "{:<10} {:<20}", "Index", "Value")?;
        writeln!(f, "{:-<30}", "")?; // Separator line

        // Iterate over the data and format each entry
        for (index, value) in self.data.iter().enumerate() {
            writeln!(f, "{:<10} {:?}", format!("_{}", index), value)?;
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn mir_locals_vec_default_initial_size() {
        let vec = MirLocalsVec::default();
        assert_eq!(vec.len(), 10);
        assert!(vec.data.iter().all(|v| matches!(v, Value::None)));
    }

    #[test]
    fn mir_locals_vec_resizes_automatically() {
        let mut vec = MirLocalsVec::default();
        vec[0] = Value::None; // no resize
        vec[5] = Value::None; // should trigger resize
        assert!(vec.len() >= 6);
    }

    #[test]
    fn mir_locals_vec_debug_formatting_works() {
        let vec = MirLocalsVec::default();
        let debug_str = format!("{vec:?}");
        assert!(debug_str.contains("Index"));
        assert!(debug_str.contains("Value"));
        assert!(debug_str.contains("_0"));
        assert!(debug_str.contains("_1"));
    }
}
