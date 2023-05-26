mod utils;

mod infinite_wait_deadlock {
    super::utils::generate_tests_for_example_program!(
        "./examples/programs/condvar/infinite_wait_deadlock.rs",
        "./examples/results/condvar/infinite_wait_deadlock/"
    );
}

mod producer_consumer {
    super::utils::generate_tests_for_example_program!(
        "./examples/programs/condvar/producer_consumer.rs",
        "./examples/results/condvar/producer_consumer/"
    );
}

mod self_notify_lost_signal {
    super::utils::generate_tests_for_example_program!(
        "./examples/programs/condvar/self_notify_lost_signal.rs",
        "./examples/results/condvar/self_notify_lost_signal/"
    );
}

mod set_condition_before_wait {
    super::utils::generate_tests_for_example_program!(
        "./examples/programs/condvar/set_condition_before_wait.rs",
        "./examples/results/condvar/set_condition_before_wait/"
    );
}

mod wait {
    super::utils::generate_tests_for_example_program!(
        "./examples/programs/condvar/wait.rs",
        "./examples/results/condvar/wait/"
    );
}

mod wait_while {
    super::utils::generate_tests_for_example_program!(
        "./examples/programs/condvar/wait_while.rs",
        "./examples/results/condvar/wait_while/"
    );
}
