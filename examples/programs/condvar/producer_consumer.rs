use std::sync::{Arc, Condvar, Mutex};
use std::thread;

/// This program implemented with only one `std::sync::Condvar`
/// generates a Petri net that requires a considerable ammount of resources
/// to be analyzed (> 16 GB RAM).
fn main() {
    let buffer = Arc::new((Mutex::new(0), Condvar::new()));

    let producer_buffer = buffer.clone();
    let consumer_buffer = buffer.clone();

    let producer = thread::spawn(move || {
        for i in 1..10 {
            let (lock, cvar) = &*producer_buffer;
            let mut buffer = lock.lock().unwrap();

            while *buffer != 0 {
                buffer = cvar.wait(buffer).unwrap();
            }

            *buffer = i;
            println!("Produced: {}", i);

            cvar.notify_one();
        }
    });

    let consumer = thread::spawn(move || loop {
        let (lock, cvar) = &*consumer_buffer;
        let mut buffer = lock.lock().unwrap();

        while *buffer == 0 {
            buffer = cvar.wait(buffer).unwrap();
        }

        let item = *buffer;
        *buffer = 0;
        println!("Consumed: {}", item);

        cvar.notify_one();
    });

    producer.join().unwrap();
    consumer.join().unwrap();
}
