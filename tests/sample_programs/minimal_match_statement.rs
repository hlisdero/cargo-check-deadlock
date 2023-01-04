fn main() {
    match std::env::args().len() {
        1 => 2,
        3 => 6,
        _ => 0,
    };
}
