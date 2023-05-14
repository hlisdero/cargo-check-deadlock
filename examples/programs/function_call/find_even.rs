fn find_even(numbers: &[i32]) -> Option<i32> {
    for &num in numbers {
        if num % 2 == 0 {
            return Some(num);
        }
    }
    None
}

fn main() {
    let numbers = vec![1, 3, 5, 2, 4, 6];

    if let Some(even) = find_even(&numbers) {
        println!("Found an even number: {}", even);
    } else {
        println!("No even number found");
    }
}
