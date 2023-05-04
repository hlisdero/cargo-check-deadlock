mod common;
use common::generate_tests_for_example_program;

generate_tests_for_example_program!(
    "./examples/programs/minimal_match_statement.rs",
    "./examples/results/minimal_match_statement/"
);
