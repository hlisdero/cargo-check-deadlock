use std::sync::Mutex;

fn try_lock_mutex_twice(mutex: Mutex<i32>) {
    let _d1 = mutex.lock();
    let _d2 = mutex.lock(); // cannot lock, since d1 is still active
}

fn main() {
    let mutex = Mutex::new(0);
    try_lock_mutex_twice(mutex);
}
