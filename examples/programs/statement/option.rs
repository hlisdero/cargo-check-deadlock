fn main() {
    let numbers = vec![1, 2, 3, 4, 5];

    let option = numbers.get(0);
    if option.is_some() {
        println!(
            "First element is valid and its value is {}",
            option.unwrap()
        );
    }
}
