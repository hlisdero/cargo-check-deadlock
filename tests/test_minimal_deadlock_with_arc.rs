mod common;
use common::generate_tests_for_example_program;

generate_tests_for_example_program!(
    "./examples/programs/minimal_deadlock_with_arc.rs",
    "./examples/results/minimal_deadlock_with_arc/"
);
