mod utils;

utils::generate_tests_for_example_program!(
    "./examples/programs/condvar/infinite_wait_deadlock.rs",
    "./examples/results/condvar/infinite_wait_deadlock/"
);
