//! A smaller version of the dining philosophers problem
//! with just two philosophers.
//!
//! The resulting Petri net is easier to visualize and explain.
//!
//! Do not use arrays, nor vectors, nor `structs`, nor `impl` blocks,
//! nor functional programming constructs to make it easier for the translator.

use std::sync::{Arc, Mutex};
use std::thread;

fn main() {
    let fork0 = Arc::new(Mutex::new(0));
    let fork1 = Arc::new(Mutex::new(1));

    let philosopher0 = {
        let left_fork = fork0.clone();
        let right_fork = fork1.clone();
        thread::spawn(move || {
            let _left = left_fork.lock().unwrap();
            let _right = right_fork.lock().unwrap();
        })
    };

    let philosopher1 = {
        let left_fork = fork1.clone();
        let right_fork = fork0.clone();
        thread::spawn(move || {
            let _left = left_fork.lock().unwrap();
            let _right = right_fork.lock().unwrap();
        })
    };

    // Wait for all threads to finish
    philosopher0.join().unwrap();
    philosopher1.join().unwrap();
}
