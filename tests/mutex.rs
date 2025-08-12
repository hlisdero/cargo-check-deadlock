mod utils;

mod double_lock_deadlock_in_function {
    super::utils::generate_tests_for_example_program!(
        double_lock_deadlock_in_function,
        "./examples/programs/mutex/double_lock_deadlock_in_function.rs",
        "./examples/results/mutex/double_lock_deadlock_in_function/"
    );
}

mod double_lock_deadlock_with_arc {
    super::utils::generate_tests_for_example_program!(
        double_lock_deadlock_with_arc,
        "./examples/programs/mutex/double_lock_deadlock_with_arc.rs",
        "./examples/results/mutex/double_lock_deadlock_with_arc/"
    );
}

mod double_lock_deadlock {
    super::utils::generate_tests_for_example_program!(
        double_lock_deadlock,
        "./examples/programs/mutex/double_lock_deadlock.rs",
        "./examples/results/mutex/double_lock_deadlock/"
    );
}

mod drop_mutex_guard_in_arc_manually {
    super::utils::generate_tests_for_example_program!(
        drop_mutex_guard_in_arc_manually,
        "./examples/programs/mutex/drop_mutex_guard_in_arc_manually.rs",
        "./examples/results/mutex/drop_mutex_guard_in_arc_manually/"
    );
}

mod drop_mutex_guard_manually {
    super::utils::generate_tests_for_example_program!(
        drop_mutex_guard_manually,
        "./examples/programs/mutex/drop_mutex_guard_manually.rs",
        "./examples/results/mutex/drop_mutex_guard_manually/"
    );
}

mod two_threads_sharing_mutex {
    super::utils::generate_tests_for_example_program!(
        two_threads_sharing_mutex,
        "./examples/programs/mutex/two_threads_sharing_mutex.rs",
        "./examples/results/mutex/two_threads_sharing_mutex/"
    );
}
