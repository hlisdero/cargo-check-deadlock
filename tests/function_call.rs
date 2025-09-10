mod utils;

mod function_call {
    super::utils::generate_tests_for_example_program!(
        diverging,
        "./examples/programs/function_call/diverging.rs",
        "./examples/results/function_call/diverging/"
    );

    super::utils::generate_tests_for_example_program!(
        double_lock_deadlock,
        "./examples/programs/function_call/double_lock_deadlock.rs",
        "./examples/results/function_call/double_lock_deadlock/"
    );

    super::utils::generate_tests_for_example_program!(
        drop_mutex_guard_manually,
        "./examples/programs/function_call/drop_mutex_guard_manually.rs",
        "./examples/results/function_call/drop_mutex_guard_manually/"
    );

    super::utils::generate_tests_for_example_program!(
        find_even,
        "./examples/programs/function_call/find_even.rs",
        "./examples/results/function_call/find_even/"
    );

    super::utils::generate_tests_for_example_program!(
        empty_function,
        "./examples/programs/function_call/empty_function.rs",
        "./examples/results/function_call/empty_function/"
    );

    super::utils::generate_tests_for_example_program!(
        in_a_loop,
        "./examples/programs/function_call/in_a_loop.rs",
        "./examples/results/function_call/in_a_loop/"
    );

    super::utils::generate_tests_for_example_program!(
        self_notify_lost_signal,
        "./examples/programs/function_call/self_notify_lost_signal.rs",
        "./examples/results/function_call/self_notify_lost_signal/"
    );

    super::utils::generate_tests_for_example_program!(
        two_calls_same_function,
        "./examples/programs/function_call/two_calls_same_function.rs",
        "./examples/results/function_call/two_calls_same_function/"
    );
}

mod lola {
    super::utils::generate_lola_tests_for_example_program!(
        diverging,
        "./examples/programs/function_call/diverging.rs",
        "./examples/results/function_call/diverging/",
        true
    );

    super::utils::generate_lola_tests_for_example_program!(
        double_lock_deadlock,
        "./examples/programs/function_call/double_lock_deadlock.rs",
        "./examples/results/function_call/double_lock_deadlock/",
        true
    );

    super::utils::generate_lola_tests_for_example_program!(
        drop_mutex_guard_manually,
        "./examples/programs/function_call/drop_mutex_guard_manually.rs",
        "./examples/results/function_call/drop_mutex_guard_manually/",
        false
    );

    super::utils::generate_lola_tests_for_example_program!(
        find_even,
        "./examples/programs/function_call/find_even.rs",
        "./examples/results/function_call/find_even/",
        false
    );

    super::utils::generate_lola_tests_for_example_program!(
        empty_function,
        "./examples/programs/function_call/empty_function.rs",
        "./examples/results/function_call/empty_function/",
        false
    );

    super::utils::generate_lola_tests_for_example_program!(
        in_a_loop,
        "./examples/programs/function_call/in_a_loop.rs",
        "./examples/results/function_call/in_a_loop/",
        false
    );

    super::utils::generate_lola_tests_for_example_program!(
        self_notify_lost_signal,
        "./examples/programs/function_call/self_notify_lost_signal.rs",
        "./examples/results/function_call/self_notify_lost_signal/",
        true
    );

    super::utils::generate_lola_tests_for_example_program!(
        two_calls_same_function,
        "./examples/programs/function_call/two_calls_same_function.rs",
        "./examples/results/function_call/two_calls_same_function/",
        false
    );
}
