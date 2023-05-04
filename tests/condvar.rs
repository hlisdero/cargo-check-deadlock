mod utils;

mod infinite_wait_deadlock {
    use super::utils;

    utils::generate_tests_for_example_program!(
        "./examples/programs/condvar/infinite_wait_deadlock.rs",
        "./examples/results/condvar/infinite_wait_deadlock/"
    );
}

mod self_notify_lost_signal {
    use super::utils;

    utils::generate_tests_for_example_program!(
        "./examples/programs/condvar/self_notify_lost_signal.rs",
        "./examples/results/condvar/self_notify_lost_signal/"
    );
}

mod two_threads_sharing_condvar {
    use super::utils;

    utils::generate_tests_for_example_program!(
        "./examples/programs/condvar/two_threads_sharing_condvar.rs",
        "./examples/results/condvar/two_threads_sharing_condvar/"
    );
}
