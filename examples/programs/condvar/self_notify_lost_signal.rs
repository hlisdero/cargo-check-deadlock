fn main() {
    let mutex = std::sync::Mutex::new(false);
    let cvar = std::sync::Condvar::new();
    let mutex_guard = mutex.lock().unwrap();
    cvar.notify_one();
    let _result = cvar.wait(mutex_guard);
}
