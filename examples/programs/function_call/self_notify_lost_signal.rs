use std::sync::{Condvar, Mutex};

fn try_self_notify(mutex: Mutex<bool>, condvar: Condvar) {
    let mutex_guard = mutex.lock().unwrap();
    condvar.notify_one();
    let _result = condvar.wait(mutex_guard);
}

fn main() {
    let mutex = Mutex::new(false);
    let condvar = Condvar::new();
    try_self_notify(mutex, condvar)
}
