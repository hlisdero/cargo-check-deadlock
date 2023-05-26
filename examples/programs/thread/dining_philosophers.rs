//! A very simple implementation of the dining philosophers problem
//!
//! Do not use arrays, nor vectors, nor `structs`, nor `impl` blocks,
//! nor functional programming constructs to make it easier for the translator.

use std::sync::{Arc, Mutex};
use std::thread;

fn main() {
    let fork0 = Arc::new(Mutex::new(0));
    let fork1 = Arc::new(Mutex::new(1));
    let fork2 = Arc::new(Mutex::new(2));
    let fork3 = Arc::new(Mutex::new(3));
    let fork4 = Arc::new(Mutex::new(4));

    let philosopher0 = {
        let left_fork = fork4.clone();
        let right_fork = fork0.clone();
        thread::spawn(move || {
            println!("Philosopher 0 is thinking.",);
            thread::sleep(std::time::Duration::from_secs(1));

            let left = left_fork.lock().unwrap();
            println!("Philosopher 0 grabbed the left fork, numbered {left}.",);
            let right = right_fork.lock().unwrap();
            println!("Philosopher 0 grabbed the right fork, numbered {right}.",);

            println!("Philosopher 0 is eating.",);
            thread::sleep(std::time::Duration::from_secs(1));

            println!("Philosopher 0 has finished dining.");
        })
    };

    let philosopher1 = {
        let left_fork = fork0.clone();
        let right_fork = fork1.clone();
        thread::spawn(move || {
            println!("Philosopher 1 is thinking.",);
            thread::sleep(std::time::Duration::from_secs(1));

            let left = left_fork.lock().unwrap();
            println!("Philosopher 1 grabbed the left fork, numbered {left}.",);
            let right = right_fork.lock().unwrap();
            println!("Philosopher 1 grabbed the right fork, numbered {right}.",);

            println!("Philosopher 1 is eating.",);
            thread::sleep(std::time::Duration::from_secs(1));

            println!("Philosopher 1 has finished dining.");
        })
    };

    let philosopher2 = {
        let left_fork = fork1.clone();
        let right_fork = fork2.clone();
        thread::spawn(move || {
            println!("Philosopher 2 is thinking.",);
            thread::sleep(std::time::Duration::from_secs(1));

            let left = left_fork.lock().unwrap();
            println!("Philosopher 2 grabbed the left fork, numbered {left}.",);
            let right = right_fork.lock().unwrap();
            println!("Philosopher 2 grabbed the right fork, numbered {right}.",);

            println!("Philosopher 2 is eating.",);
            thread::sleep(std::time::Duration::from_secs(1));

            println!("Philosopher 2 has finished dining.");
        })
    };

    let philosopher3 = {
        let left_fork = fork2.clone();
        let right_fork = fork3.clone();
        thread::spawn(move || {
            println!("Philosopher 3 is thinking.",);
            thread::sleep(std::time::Duration::from_secs(1));

            let left = left_fork.lock().unwrap();
            println!("Philosopher 3 grabbed the left fork, numbered {left}.",);
            let right = right_fork.lock().unwrap();
            println!("Philosopher 3 grabbed the right fork, numbered {right}.",);

            println!("Philosopher 3 is eating.",);
            thread::sleep(std::time::Duration::from_secs(1));

            println!("Philosopher 3 has finished dining.");
        })
    };

    let philosopher4 = {
        let left_fork = fork3.clone();
        let right_fork = fork4.clone();
        thread::spawn(move || {
            println!("Philosopher 4 is thinking.",);
            thread::sleep(std::time::Duration::from_secs(1));

            let left = left_fork.lock().unwrap();
            println!("Philosopher 4 grabbed the left fork, numbered {left}.",);
            let right = right_fork.lock().unwrap();
            println!("Philosopher 4 grabbed the right fork, numbered {right}.",);

            println!("Philosopher 4 is eating.",);
            thread::sleep(std::time::Duration::from_secs(1));

            println!("Philosopher 4 has finished dining.");
        })
    };

    // Wait for all threads to finish
    philosopher0.join().unwrap();
    philosopher1.join().unwrap();
    philosopher2.join().unwrap();
    philosopher3.join().unwrap();
    philosopher4.join().unwrap();
}
