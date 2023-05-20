use std::sync::{Arc, Condvar, Mutex};
use std::thread;

/// Using two `std::sync::Condvar` to make the Petri net more straightforward
/// and reduce dependencies between the markings.
fn main() {
    let buffer = Arc::new((Mutex::new(0), Condvar::new(), Condvar::new()));

    let producer_buffer = buffer.clone();
    let consumer_buffer = buffer.clone();

    let producer = thread::spawn(move || {
        for i in 1..10 {
            let (lock, cvar_producer, cvar_consumer) = &*producer_buffer;
            let mut buffer = lock.lock().unwrap();

            while *buffer != 0 {
                buffer = cvar_producer.wait(buffer).unwrap();
            }

            *buffer = i;
            println!("Produced: {}", i);

            cvar_consumer.notify_one();
        }
    });

    let consumer = thread::spawn(move || loop {
        let (lock, cvar_producer, cvar_consumer) = &*consumer_buffer;
        let mut buffer = lock.lock().unwrap();

        while *buffer == 0 {
            buffer = cvar_consumer.wait(buffer).unwrap();
        }

        let item = *buffer;
        *buffer = 0;
        println!("Consumed: {}", item);

        cvar_producer.notify_one();
    });

    producer.join().unwrap();
    consumer.join().unwrap();
}
