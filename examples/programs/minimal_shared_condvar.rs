//! Example taken from the documentation:
//! <https://doc.rust-lang.org/std/sync/struct.Condvar.html>
fn main() {
    let pair = std::sync::Arc::new((std::sync::Mutex::new(false), std::sync::Condvar::new()));
    let pair2 = std::sync::Arc::clone(&pair);

    // Inside of our lock, spawn a new thread, and then wait for it to start.
    std::thread::spawn(move || {
        let (lock, cvar) = &*pair2;
        let mut started = lock.lock().unwrap();
        *started = true;
        // We notify the condvar that the value has changed.
        cvar.notify_one();
    });

    // Wait for the thread to start up.
    let (lock, cvar) = &*pair;
    let mut started = lock.lock().unwrap();
    while !*started {
        started = cvar.wait(started).unwrap();
    }
}
