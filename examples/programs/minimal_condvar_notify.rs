fn main() {
    let lock = std::sync::Mutex::new(false);
    let cvar = std::sync::Condvar::new();
    let lock_guard = lock.lock().unwrap();
    cvar.notify_one();
    let _result = cvar.wait(lock_guard);
}
