fn main() {
    let data = std::sync::Mutex::new(0);
    let d1 = data.lock();
    std::mem::drop(d1);
    let _d2 = data.lock(); // can lock, since d1 was dropped manually
}
