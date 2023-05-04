mod common;
use common::generate_tests_for_example_program;

generate_tests_for_example_program!(
    "./examples/programs/two_mutexes_two_functions.rs",
    "./examples/results/two_mutexes_two_functions/"
);
