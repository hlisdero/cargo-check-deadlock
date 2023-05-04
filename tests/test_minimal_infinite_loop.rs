mod common;
use common::generate_tests_for_example_program;

generate_tests_for_example_program!(
    "./examples/programs/minimal_infinite_loop.rs",
    "./examples/results/minimal_infinite_loop/"
);
