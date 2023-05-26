fn main() {
    let original_data = std::sync::Arc::new(std::sync::Mutex::new(0));
    let copy_data = original_data.clone();

    let thread_join_handle = std::thread::spawn(move || {
        let _data = copy_data.lock();
    });

    let data = original_data.lock();
    // Drop the lock manually before the join
    std::mem::drop(data);
    let _join_result = thread_join_handle.join();
}
