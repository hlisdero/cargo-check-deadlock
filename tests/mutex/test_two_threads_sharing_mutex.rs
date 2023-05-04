mod utils;

utils::generate_tests_for_example_program!(
    "./examples/programs/mutex/two_threads_sharing_mutex.rs",
    "./examples/results/mutex/two_threads_sharing_mutex/"
);
