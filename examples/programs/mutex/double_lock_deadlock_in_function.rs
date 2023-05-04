fn first_deadlock() {
    let data = std::sync::Mutex::new(0);
    let _d1 = data.lock();
    let _d2 = data.lock(); // cannot lock, since d1 is still active
}

fn second_deadlock() {
    let data = std::sync::Mutex::new(0);
    let _d1 = data.lock();
    let _d2 = data.lock(); // cannot lock, since d1 is still active
}

fn main() {
    first_deadlock();
    second_deadlock();
}
