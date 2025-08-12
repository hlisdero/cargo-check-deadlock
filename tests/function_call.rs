mod utils;

mod function_call {
    super::utils::generate_tests_for_example_program!(
        diverging,
        "./examples/programs/function_call/diverging.rs",
        "./examples/results/function_call/diverging/"
    );

    super::utils::generate_tests_for_example_program!(
        find_even,
        "./examples/programs/function_call/find_even.rs",
        "./examples/results/function_call/find_even/"
    );

    super::utils::generate_tests_for_example_program!(
        empty_function,
        "./examples/programs/function_call/empty_function.rs",
        "./examples/results/function_call/empty_function/"
    );

    super::utils::generate_tests_for_example_program!(
        in_a_loop,
        "./examples/programs/function_call/in_a_loop.rs",
        "./examples/results/function_call/in_a_loop/"
    );

    super::utils::generate_tests_for_example_program!(
        two_calls_same_function,
        "./examples/programs/function_call/two_calls_same_function.rs",
        "./examples/results/function_call/two_calls_same_function/"
    );
}

mod lola {
    super::utils::generate_lola_tests_for_example_program!(
        diverging,
        "./examples/programs/function_call/diverging.rs",
        "./examples/results/function_call/diverging/",
        true
    );

    super::utils::generate_lola_tests_for_example_program!(
        find_even,
        "./examples/programs/function_call/find_even.rs",
        "./examples/results/function_call/find_even/",
        false
    );

    super::utils::generate_lola_tests_for_example_program!(
        empty_function,
        "./examples/programs/function_call/empty_function.rs",
        "./examples/results/function_call/empty_function/",
        false
    );

    super::utils::generate_lola_tests_for_example_program!(
        in_a_loop,
        "./examples/programs/function_call/in_a_loop.rs",
        "./examples/results/function_call/in_a_loop/",
        false
    );

    super::utils::generate_lola_tests_for_example_program!(
        two_calls_same_function,
        "./examples/programs/function_call/two_calls_same_function.rs",
        "./examples/results/function_call/two_calls_same_function/",
        false
    );
}
