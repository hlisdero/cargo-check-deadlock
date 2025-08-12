mod utils;

mod condvar {
    super::utils::generate_tests_for_example_program!(
        infinite_wait_deadlock,
        "./examples/programs/condvar/infinite_wait_deadlock.rs",
        "./examples/results/condvar/infinite_wait_deadlock/"
    );

    super::utils::generate_tests_for_example_program!(
        producer_consumer,
        "./examples/programs/condvar/producer_consumer.rs",
        "./examples/results/condvar/producer_consumer/"
    );

    super::utils::generate_tests_for_example_program!(
        self_notify_lost_signal,
        "./examples/programs/condvar/self_notify_lost_signal.rs",
        "./examples/results/condvar/self_notify_lost_signal/"
    );

    super::utils::generate_tests_for_example_program!(
        set_condition_before_wait,
        "./examples/programs/condvar/set_condition_before_wait.rs",
        "./examples/results/condvar/set_condition_before_wait/"
    );

    super::utils::generate_tests_for_example_program!(
        wait,
        "./examples/programs/condvar/wait.rs",
        "./examples/results/condvar/wait/"
    );

    super::utils::generate_tests_for_example_program!(
        wait_while,
        "./examples/programs/condvar/wait_while.rs",
        "./examples/results/condvar/wait_while/"
    );
}

mod lola {
    super::utils::generate_lola_tests_for_example_program!(
        infinite_wait_deadlock,
        "./examples/programs/condvar/infinite_wait_deadlock.rs",
        "./examples/results/condvar/infinite_wait_deadlock/",
        true
    );

    super::utils::generate_lola_tests_for_example_program!(
        producer_consumer,
        "./examples/programs/condvar/producer_consumer.rs",
        "./examples/results/condvar/producer_consumer/",
        false
    );

    super::utils::generate_lola_tests_for_example_program!(
        self_notify_lost_signal,
        "./examples/programs/condvar/self_notify_lost_signal.rs",
        "./examples/results/condvar/self_notify_lost_signal/",
        true
    );

    super::utils::generate_lola_tests_for_example_program!(
        set_condition_before_wait,
        "./examples/programs/condvar/set_condition_before_wait.rs",
        "./examples/results/condvar/set_condition_before_wait/",
        false
    );

    super::utils::generate_lola_tests_for_example_program!(
        wait,
        "./examples/programs/condvar/wait.rs",
        "./examples/results/condvar/wait/",
        false
    );

    super::utils::generate_lola_tests_for_example_program!(
        wait_while,
        "./examples/programs/condvar/wait_while.rs",
        "./examples/results/condvar/wait_while/",
        false
    );
}
