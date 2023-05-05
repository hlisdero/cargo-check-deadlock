fn main() {
    // Get the command-line arguments
    let args: Vec<String> = std::env::args().collect();

    // Check that we received one argument (the name)
    if args.len() != 2 {
        eprintln!("Usage: {} <name>", args[0]);
        std::process::exit(1);
    }

    // Get the name from the arguments
    let name = &args[1];

    // Print the greeting
    println!("Hello, {}!", name);
}
