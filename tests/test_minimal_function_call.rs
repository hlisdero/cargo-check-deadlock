mod common;
use common::generate_tests_for_example_program;

generate_tests_for_example_program!(
    "./examples/programs/minimal_function_call.rs",
    "./examples/results/minimal_function_call/"
);
