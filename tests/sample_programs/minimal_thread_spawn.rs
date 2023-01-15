fn main() {
    let thread_join_handle = std::thread::spawn(move || {
        // some work here
    });
    // some work here
    let _res = thread_join_handle.join();
}
