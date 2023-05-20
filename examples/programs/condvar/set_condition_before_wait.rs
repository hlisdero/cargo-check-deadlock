//! This is a modification of the example taken from the documentation:
//! <https://doc.rust-lang.org/std/sync/struct.Condvar.html>
//!
//! Its purpose is to showcase the importance of the order in which the statement that
//! sets the condition for the condition variable and the call to `wait()` appear.
//!
//! As the tool scans the Rust code, it translate first the main thread. We would like
//! the statement `*started = true` to appear here. Later, the waiting thread is translated.
//! The call to `wait()` will then have already seen that condition was set.
//! This is the simplest version possible for the translator.

use std::sync::{Arc, Condvar, Mutex};
use std::thread;

fn main() {
    let pair = Arc::new((Mutex::new(false), Condvar::new()));
    let pair2 = Arc::clone(&pair);

    // Inside of our lock, spawn a new thread that waits until we set the condition
    let waiting = thread::spawn(move || {
        let (lock, cvar) = &*pair;
        // As long as the value inside the `Mutex<bool>` is `true`, we wait.
        let _guard = cvar
            .wait_while(lock.lock().unwrap(), |pending| *pending)
            .unwrap();
    });

    let (lock, cvar) = &*pair2;
    let mut pending = lock.lock().unwrap();
    *pending = false;
    // We notify the condvar that the value has changed.
    cvar.notify_one();

    // Drop the lock manually before the join
    std::mem::drop(pending);
    // Join the pending thread
    waiting.join().unwrap();
}
