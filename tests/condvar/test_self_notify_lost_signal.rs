mod utils;

utils::generate_tests_for_example_program!(
    "./examples/programs/condvar/self_notify_lost_signal.rs",
    "./examples/results/condvar/self_notify_lost_signal/"
);
