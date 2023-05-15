use std::sync::{Arc, Mutex};
use std::thread;

fn main() {
    // Create a shared counter wrapped in a mutex
    let counter = Arc::new(Mutex::new(0));

    // Clone the shared counter for each thread
    let counter1 = Arc::clone(&counter);
    let counter2 = Arc::clone(&counter);

    // Spawn the first thread
    let thread1 = thread::spawn(move || loop {
        let mut num = counter1.lock().unwrap();
        if *num >= 1000 {
            break;
        }
        *num += 1;
    });

    // Spawn the second thread
    let thread2 = thread::spawn(move || loop {
        let mut num = counter2.lock().unwrap();
        if *num >= 1000 {
            break;
        }
        *num += 1;
    });

    // Wait for both threads to finish
    thread1.join().unwrap();
    thread2.join().unwrap();

    // Print the final value of the counter
    println!("Final counter value: {}", *counter.lock().unwrap());
}
