mod utils;

mod mutex {
    super::utils::generate_tests_for_example_program!(
        double_lock_deadlock_in_function,
        "./examples/programs/mutex/double_lock_deadlock_in_function.rs",
        "./examples/results/mutex/double_lock_deadlock_in_function/"
    );

    super::utils::generate_tests_for_example_program!(
        double_lock_deadlock_with_arc,
        "./examples/programs/mutex/double_lock_deadlock_with_arc.rs",
        "./examples/results/mutex/double_lock_deadlock_with_arc/"
    );

    super::utils::generate_tests_for_example_program!(
        double_lock_deadlock,
        "./examples/programs/mutex/double_lock_deadlock.rs",
        "./examples/results/mutex/double_lock_deadlock/"
    );

    super::utils::generate_tests_for_example_program!(
        drop_mutex_guard_in_arc_manually,
        "./examples/programs/mutex/drop_mutex_guard_in_arc_manually.rs",
        "./examples/results/mutex/drop_mutex_guard_in_arc_manually/"
    );

    super::utils::generate_tests_for_example_program!(
        drop_mutex_guard_manually,
        "./examples/programs/mutex/drop_mutex_guard_manually.rs",
        "./examples/results/mutex/drop_mutex_guard_manually/"
    );

    super::utils::generate_tests_for_example_program!(
        two_threads_sharing_mutex,
        "./examples/programs/mutex/two_threads_sharing_mutex.rs",
        "./examples/results/mutex/two_threads_sharing_mutex/"
    );
}

mod lola {
    super::utils::generate_lola_tests_for_example_program!(
        double_lock_deadlock_in_function,
        "./examples/programs/mutex/double_lock_deadlock_in_function.rs",
        "./examples/results/mutex/double_lock_deadlock_in_function/",
        true
    );

    super::utils::generate_lola_tests_for_example_program!(
        double_lock_deadlock_with_arc,
        "./examples/programs/mutex/double_lock_deadlock_with_arc.rs",
        "./examples/results/mutex/double_lock_deadlock_with_arc/",
        true
    );

    super::utils::generate_lola_tests_for_example_program!(
        double_lock_deadlock,
        "./examples/programs/mutex/double_lock_deadlock.rs",
        "./examples/results/mutex/double_lock_deadlock/",
        true
    );

    super::utils::generate_lola_tests_for_example_program!(
        drop_mutex_guard_in_arc_manually,
        "./examples/programs/mutex/drop_mutex_guard_in_arc_manually.rs",
        "./examples/results/mutex/drop_mutex_guard_in_arc_manually/",
        false
    );

    super::utils::generate_lola_tests_for_example_program!(
        drop_mutex_guard_manually,
        "./examples/programs/mutex/drop_mutex_guard_manually.rs",
        "./examples/results/mutex/drop_mutex_guard_manually/",
        false
    );

    super::utils::generate_lola_tests_for_example_program!(
        two_threads_sharing_mutex,
        "./examples/programs/mutex/two_threads_sharing_mutex.rs",
        "./examples/results/mutex/two_threads_sharing_mutex/",
        false
    );
}
