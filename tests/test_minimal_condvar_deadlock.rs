mod common;
use common::generate_tests_for_example_program;

generate_tests_for_example_program!(
    "./examples/programs/minimal_condvar_deadlock.rs",
    "./examples/results/minimal_condvar_deadlock/"
);
