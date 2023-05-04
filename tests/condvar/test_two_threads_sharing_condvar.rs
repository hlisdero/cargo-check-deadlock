mod utils;

utils::generate_tests_for_example_program!(
    "./examples/programs/condvar/two_threads_sharing_condvar.rs",
    "./examples/results/condvar/two_threads_sharing_condvar/"
);
