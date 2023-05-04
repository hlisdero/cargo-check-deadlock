mod utils;

mod double_lock_deadlock_in_function {
    use super::utils;

    utils::generate_tests_for_example_program!(
        "./examples/programs/mutex/double_lock_deadlock_in_function.rs",
        "./examples/results/mutex/double_lock_deadlock_in_function/"
    );
}

mod double_lock_deadlock_with_arc {
    use super::utils;

    utils::generate_tests_for_example_program!(
        "./examples/programs/mutex/double_lock_deadlock_with_arc.rs",
        "./examples/results/mutex/double_lock_deadlock_with_arc/"
    );
}

mod double_lock_deadlock {
    use super::utils;

    utils::generate_tests_for_example_program!(
        "./examples/programs/mutex/double_lock_deadlock.rs",
        "./examples/results/mutex/double_lock_deadlock/"
    );
}

mod two_threads_sharing_mutex {
    use super::utils;

    utils::generate_tests_for_example_program!(
        "./examples/programs/mutex/two_threads_sharing_mutex.rs",
        "./examples/results/mutex/two_threads_sharing_mutex/"
    );
}
