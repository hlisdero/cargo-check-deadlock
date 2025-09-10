use std::sync::{Arc, Mutex};

fn try_drop_mutex_guard_manually(mutex: Arc<Mutex<i32>>) {
    let d1 = mutex.lock();
    std::mem::drop(d1);
    let _d2 = mutex.lock(); // can lock, since d1 was dropped manually
}

fn main() {
    let mutex = Arc::new(Mutex::new(0));
    try_drop_mutex_guard_manually(mutex);
}
