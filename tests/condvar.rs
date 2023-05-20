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

mod wait {
    use super::utils;

    utils::generate_tests_for_example_program!(
        "./examples/programs/condvar/wait.rs",
        "./examples/results/condvar/wait/"
    );
}

mod wait_while {
    use super::utils;

    utils::generate_tests_for_example_program!(
        "./examples/programs/condvar/wait_while.rs",
        "./examples/results/condvar/wait_while/"
    );
}
