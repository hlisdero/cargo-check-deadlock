use std::sync::{Arc, Mutex};

struct Data {
    value: Mutex<i32>,
}

fn main() {
    let data = Arc::new(Data {
        value: Mutex::new(0),
    });

    let _d1 = data.value.lock();
    let _d2 = data.value.lock();
}