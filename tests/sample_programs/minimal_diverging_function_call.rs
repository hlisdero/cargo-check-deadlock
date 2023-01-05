fn does_not_return() -> ! {
    loop {}
}

fn main() {
    does_not_return()
}
