fn main() {
    let thread_join_handle = std::thread::spawn(move || {
        // never returns
        loop {}
    });
    // some work here
    let _res = thread_join_handle.join();
}
