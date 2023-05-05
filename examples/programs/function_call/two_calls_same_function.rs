fn simple_function() {}

#[allow(unused_assignments)]
pub fn main() {
    let mut second_call = false;
    simple_function();
    if second_call {
        panic!()
    }
    second_call = true;
    simple_function();
}
