mod utils;

mod calculator {
    super::utils::generate_tests_for_example_program!(
        calculator,
        "./examples/programs/basic/calculator.rs",
        "./examples/results/basic/calculator/"
    );
}

mod greet {
    super::utils::generate_tests_for_example_program!(
        greet,
        "./examples/programs/basic/greet.rs",
        "./examples/results/basic/greet/"
    );
}

mod hello_world {
    super::utils::generate_tests_for_example_program!(
        hello_world,
        "./examples/programs/basic/hello_world.rs",
        "./examples/results/basic/hello_world/"
    );
}
