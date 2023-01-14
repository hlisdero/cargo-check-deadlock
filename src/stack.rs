//! Submodule that implements a simple stack data structure.
//!
//! It is used to implement the call stack for the source code translation.

/// Error message when the stack is empty.
const EMPTY_STACK: &str = "BUG: `peek_mut` should not be called on an empty stack";

pub struct Stack<T> {
    stack: Vec<T>,
}

impl<T> Stack<T> {
    /// Creates a new empty stack.
    #[must_use]
    pub const fn new() -> Self {
        Self { stack: Vec::new() }
    }

    /// Pushes an element to the top of the stack.
    #[inline]
    pub fn push(&mut self, item: T) {
        self.stack.push(item);
    }

    /// Removes and returns the top element of the stack.
    #[inline]
    pub fn pop(&mut self) -> Option<T> {
        self.stack.pop()
    }

    /// Returns a mutable reference to the top element of the stack.
    /// Does not remove the element from the stack.
    ///
    /// # Panics
    ///
    /// If the stack is empty, then the function panics.
    pub fn peek_mut(&mut self) -> &mut T {
        if self.stack.is_empty() {
            panic!("{EMPTY_STACK}");
        } else {
            let len = self.stack.len();
            &mut self.stack[len - 1]
        }
    }
}

#[cfg(test)]
mod stack_tests {
    use super::*;

    #[test]
    fn stack_new_is_empty() {
        let stack: Stack<usize> = Stack::new();

        assert!(stack.stack.is_empty());
    }

    #[test]
    fn stack_new_has_length_zero() {
        let stack: Stack<usize> = Stack::new();

        assert_eq!(stack.stack.len(), 0);
    }

    #[test]
    fn stack_new_pop_returns_none() {
        let mut stack: Stack<usize> = Stack::new();

        assert!(stack.pop().is_none());
    }

    #[test]
    #[should_panic(expected = "BUG: `peek_mut` should not be called on an empty stack")]
    fn stack_new_peek_mut_returns_none() {
        let mut stack: Stack<usize> = Stack::new();

        stack.peek_mut();
    }

    #[test]
    fn stack_push_updates_length() {
        let mut stack: Stack<usize> = Stack::new();
        stack.push(0);

        assert!(!stack.stack.is_empty());
        assert_eq!(stack.stack.len(), 1);
    }

    #[test]
    fn stack_push_lots_of_elements() {
        let mut stack: Stack<usize> = Stack::new();
        assert_eq!(stack.stack.len(), 0);

        for i in 0..10 {
            stack.push(i);
        }

        assert!(!stack.stack.is_empty());
        assert_eq!(stack.stack.len(), 10);
    }

    #[test]
    fn stack_pop_updates_length() {
        let mut stack: Stack<usize> = Stack::new();

        stack.push(1);
        let result = stack.pop();

        assert!(result.is_some());
        assert_eq!(result.unwrap(), 1);
        assert!(stack.stack.is_empty());
    }

    #[test]
    fn stack_pop_returns_none_if_empty() {
        let mut stack: Stack<usize> = Stack::new();

        stack.push(1);
        stack.pop();
        let result = stack.pop();

        assert!(result.is_none());
        assert!(stack.stack.is_empty());
        assert_eq!(stack.stack.len(), 0)
    }

    #[test]
    fn stack_pop_lots_of_elements() {
        let mut stack: Stack<usize> = Stack::new();
        assert_eq!(stack.stack.len(), 0);

        for i in 0..10 {
            stack.push(i);
        }

        for _ in 0..7 {
            stack.pop();
        }

        assert!(!stack.stack.is_empty());
        assert_eq!(stack.stack.len(), 3);
    }

    #[test]
    fn stack_peek_mut_returns_top_element() {
        let mut stack: Stack<usize> = Stack::new();

        stack.push(101);
        let result = stack.peek_mut();

        assert_eq!(*result, 101);
        assert!(!stack.stack.is_empty());
    }

    #[test]
    fn stack_peek_mut_allows_modification_of_top_element() {
        let mut stack: Stack<usize> = Stack::new();

        stack.push(101);
        let top = stack.peek_mut();
        *top = 999;
        let result = stack.peek_mut();

        assert_eq!(*result, 999);
        assert!(!stack.stack.is_empty());
    }
}
