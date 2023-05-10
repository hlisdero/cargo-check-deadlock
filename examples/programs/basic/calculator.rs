fn main() {
    // Get the command-line arguments
    let args: Vec<String> = std::env::args().collect();

    // Check that we received three arguments (num1, num2, operator)
    if args.len() != 4 {
        eprintln!("Usage: {} <num1> <num2> <operator>", args[0]);
        std::process::exit(1);
    }

    // Parse the numbers
    let num1: f64 = args[1].parse().expect("Invalid number");
    let num2: f64 = args[2].parse().expect("Invalid number");

    // Get the operator from the arguments
    let operator = &args[3];

    // Perform the arithmetic operation and print the result
    let result = match operator.as_str() {
        "+" => num1 + num2,
        "-" => num1 - num2,
        "*" => num1 * num2,
        "/" => num1 / num2,
        _ => {
            eprintln!("Invalid operator: {}", operator);
            std::process::exit(1);
        }
    };

    println!("Result: {}", result);
}
