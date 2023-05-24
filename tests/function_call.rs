mod utils;

mod diverging {
    super::utils::generate_tests_for_example_program!(
        "./examples/programs/function_call/diverging.rs",
        "./examples/results/function_call/diverging/"
    );
}

mod find_even {
    super::utils::generate_tests_for_example_program!(
        "./examples/programs/function_call/find_even.rs",
        "./examples/results/function_call/find_even/"
    );
}

mod empty_function {
    super::utils::generate_tests_for_example_program!(
        "./examples/programs/function_call/empty_function.rs",
        "./examples/results/function_call/empty_function/"
    );
}

mod in_a_loop {
    super::utils::generate_tests_for_example_program!(
        "./examples/programs/function_call/in_a_loop.rs",
        "./examples/results/function_call/in_a_loop/"
    );
}

mod two_calls_same_function {
    super::utils::generate_tests_for_example_program!(
        "./examples/programs/function_call/two_calls_same_function.rs",
        "./examples/results/function_call/two_calls_same_function/"
    );
}
