//! Submodule that implements a simple stack data structure.
//!
//! It is used to implement the call stack for the source code translation.

struct Stack<T> {
    stack: Vec<T>,
}

impl<T> Stack<T> {
    /// Create a new empty stack.
    #[must_use]
    pub fn new() -> Self {
        Stack { stack: Vec::new() }
    }

    /// Push an element to the top of the stack.
    #[inline]
    pub fn push(&mut self, item: T) {
        self.stack.push(item)
    }

    /// Remove and return the top element of the stack.
    #[inline]
    pub fn pop(&mut self) -> Option<T> {
        self.stack.pop()
    }

    /// Return an immutable reference to the top element of the stack.
    /// Does not remove the element from the stack.
    pub fn peek(&self) -> Option<&T> {
        if self.stack.is_empty() {
            None
        } else {
            Some(&self.stack[self.stack.len() - 1])
        }
    }

    /// Return a mutable reference to the top element of the stack.
    /// Does not remove the element from the stack.
    pub fn peek_mut(&mut self) -> Option<&mut T> {
        if self.stack.is_empty() {
            None
        } else {
            let len = self.stack.len();
            Some(&mut self.stack[len - 1])
        }
    }

    /// Check whether the stack is empty.
    #[inline]
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.stack.is_empty()
    }

    /// Return how many elements the stack has.
    #[inline]
    #[must_use]
    pub fn len(&self) -> usize {
        self.stack.len()
    }
}

#[cfg(test)]
mod stack_tests {
    use super::*;

    #[test]
    fn stack_new_is_empty() {
        let stack: Stack<usize> = Stack::new();

        assert!(stack.is_empty());
    }

    #[test]
    fn stack_new_has_length_zero() {
        let stack: Stack<usize> = Stack::new();

        assert_eq!(stack.len(), 0);
    }

    #[test]
    fn stack_new_pop_returns_none() {
        let mut stack: Stack<usize> = Stack::new();

        assert!(stack.pop().is_none());
    }

    #[test]
    fn stack_new_peek_returns_none() {
        let stack: Stack<usize> = Stack::new();

        assert!(stack.peek().is_none());
    }

    #[test]
    fn stack_new_peek_mut_returns_none() {
        let mut stack: Stack<usize> = Stack::new();

        assert!(stack.peek_mut().is_none());
    }

    #[test]
    fn stack_push_updates_length() {
        let mut stack: Stack<usize> = Stack::new();
        stack.push(0);

        assert!(!stack.is_empty());
        assert_eq!(stack.len(), 1);
    }

    #[test]
    fn stack_push_lots_of_elements() {
        let mut stack: Stack<usize> = Stack::new();
        assert_eq!(stack.len(), 0);

        for i in 0..10 {
            stack.push(i);
        }

        assert!(!stack.is_empty());
        assert_eq!(stack.len(), 10);
    }

    #[test]
    fn stack_pop_updates_length() {
        let mut stack: Stack<usize> = Stack::new();

        stack.push(1);
        let result = stack.pop();

        assert!(result.is_some());
        assert_eq!(result.unwrap(), 1);
        assert!(stack.is_empty());
    }

    #[test]
    fn stack_pop_returns_none_if_empty() {
        let mut stack: Stack<usize> = Stack::new();

        stack.push(1);
        stack.pop();
        let result = stack.pop();

        assert!(result.is_none());
        assert!(stack.is_empty());
        assert_eq!(stack.len(), 0)
    }

    #[test]
    fn stack_pop_lots_of_elements() {
        let mut stack: Stack<usize> = Stack::new();
        assert_eq!(stack.len(), 0);

        for i in 0..10 {
            stack.push(i);
        }

        for _ in 0..7 {
            stack.pop();
        }

        assert!(!stack.is_empty());
        assert_eq!(stack.len(), 3);
    }

    #[test]
    fn stack_peek_returns_top_element() {
        let mut stack: Stack<usize> = Stack::new();

        stack.push(101);
        let result = stack.peek();

        assert!(result.is_some());
        assert_eq!(*result.unwrap(), 101);
        assert!(!stack.is_empty());
    }

    #[test]
    fn stack_peek_mut_returns_top_element() {
        let mut stack: Stack<usize> = Stack::new();

        stack.push(101);
        let result = stack.peek_mut();

        assert!(result.is_some());
        assert_eq!(*result.unwrap(), 101);
        assert!(!stack.is_empty());
    }

    #[test]
    fn stack_peek_mut_allows_modification_of_top_element() {
        let mut stack: Stack<usize> = Stack::new();

        stack.push(101);
        let top = stack.peek_mut().unwrap();
        *top = 999;
        let result = stack.peek();

        assert!(result.is_some());
        assert_eq!(*result.unwrap(), 999);
        assert!(!stack.is_empty());
    }
}
