//! Tests for the model checker `LoLA`
//!
//! Check that the results matches the expected output for every program.

mod utils;

mod calculator {
    use super::utils;

    utils::generate_lola_tests_for_example_program!(
        "./examples/programs/basic/calculator.rs",
        "./examples/results/basic/calculator/",
        false
    );
}

mod greet {
    use super::utils;

    utils::generate_lola_tests_for_example_program!(
        "./examples/programs/basic/greet.rs",
        "./examples/results/basic/greet/",
        false
    );
}

mod hello_world {
    use super::utils;

    utils::generate_lola_tests_for_example_program!(
        "./examples/programs/basic/hello_world.rs",
        "./examples/results/basic/hello_world/",
        false
    );
}

mod infinite_wait_deadlock {
    use super::utils;

    utils::generate_lola_tests_for_example_program!(
        "./examples/programs/condvar/infinite_wait_deadlock.rs",
        "./examples/results/condvar/infinite_wait_deadlock/",
        true
    );
}

mod self_notify_lost_signal {
    use super::utils;

    utils::generate_lola_tests_for_example_program!(
        "./examples/programs/condvar/self_notify_lost_signal.rs",
        "./examples/results/condvar/self_notify_lost_signal/",
        true
    );
}

// mod two_threads_sharing_condvar {
//     use super::utils;
//
//     utils::generate_lola_tests_for_example_program!(
//         "./examples/programs/condvar/two_threads_sharing_condvar.rs",
//         "./examples/results/condvar/two_threads_sharing_condvar/",
//         false
//     );
// }

mod diverging {
    use super::utils;

    utils::generate_lola_tests_for_example_program!(
        "./examples/programs/function_call/diverging.rs",
        "./examples/results/function_call/diverging/",
        true
    );
}

mod find_even {
    use super::utils;

    utils::generate_lola_tests_for_example_program!(
        "./examples/programs/function_call/find_even.rs",
        "./examples/results/function_call/find_even/",
        false
    );
}

mod empty_function {
    use super::utils;

    utils::generate_lola_tests_for_example_program!(
        "./examples/programs/function_call/empty_function.rs",
        "./examples/results/function_call/empty_function/",
        false
    );
}

mod in_a_loop {
    use super::utils;

    utils::generate_lola_tests_for_example_program!(
        "./examples/programs/function_call/in_a_loop.rs",
        "./examples/results/function_call/in_a_loop/",
        false
    );
}

mod two_calls_same_function {
    use super::utils;

    utils::generate_lola_tests_for_example_program!(
        "./examples/programs/function_call/two_calls_same_function.rs",
        "./examples/results/function_call/two_calls_same_function/",
        false
    );
}

mod double_lock_deadlock_in_function {
    use super::utils;

    utils::generate_lola_tests_for_example_program!(
        "./examples/programs/mutex/double_lock_deadlock_in_function.rs",
        "./examples/results/mutex/double_lock_deadlock_in_function/",
        true
    );
}

mod double_lock_deadlock_with_arc {
    use super::utils;

    utils::generate_lola_tests_for_example_program!(
        "./examples/programs/mutex/double_lock_deadlock_with_arc.rs",
        "./examples/results/mutex/double_lock_deadlock_with_arc/",
        true
    );
}

mod double_lock_deadlock {
    use super::utils;

    utils::generate_lola_tests_for_example_program!(
        "./examples/programs/mutex/double_lock_deadlock.rs",
        "./examples/results/mutex/double_lock_deadlock/",
        true
    );
}

mod drop_mutex_guard_in_arc_manually {
    use super::utils;

    utils::generate_lola_tests_for_example_program!(
        "./examples/programs/mutex/drop_mutex_guard_in_arc_manually.rs",
        "./examples/results/mutex/drop_mutex_guard_in_arc_manually/",
        false
    );
}

mod drop_mutex_guard_manually {
    use super::utils;

    utils::generate_lola_tests_for_example_program!(
        "./examples/programs/mutex/drop_mutex_guard_manually.rs",
        "./examples/results/mutex/drop_mutex_guard_manually/",
        false
    );
}

mod two_threads_sharing_mutex {
    use super::utils;

    utils::generate_lola_tests_for_example_program!(
        "./examples/programs/mutex/two_threads_sharing_mutex.rs",
        "./examples/results/mutex/two_threads_sharing_mutex/",
        false
    );
}

mod abort {
    use super::utils;

    utils::generate_lola_tests_for_example_program!(
        "./examples/programs/statement/abort.rs",
        "./examples/results/statement/abort/",
        true
    );
}

mod empty_main {
    use super::utils;

    utils::generate_lola_tests_for_example_program!(
        "./examples/programs/statement/empty_main.rs",
        "./examples/results/statement/empty_main/",
        false
    );
}

mod infinite_loop {
    use super::utils;

    utils::generate_lola_tests_for_example_program!(
        "./examples/programs/statement/infinite_loop.rs",
        "./examples/results/statement/infinite_loop/",
        false
    );
}

mod match_stmt {
    use super::utils;

    utils::generate_lola_tests_for_example_program!(
        "./examples/programs/statement/match.rs",
        "./examples/results/statement/match/",
        false
    );
}

mod option {
    use super::utils;

    utils::generate_lola_tests_for_example_program!(
        "./examples/programs/statement/option.rs",
        "./examples/results/statement/option/",
        false
    );
}

mod panic {
    use super::utils;

    utils::generate_lola_tests_for_example_program!(
        "./examples/programs/statement/panic.rs",
        "./examples/results/statement/panic/",
        false
    );
}

mod detached {
    use super::utils;

    utils::generate_lola_tests_for_example_program!(
        "./examples/programs/thread/detached.rs",
        "./examples/results/thread/detached/",
        false
    );
}

mod diverging_thread {
    use super::utils;

    utils::generate_lola_tests_for_example_program!(
        "./examples/programs/thread/diverging.rs",
        "./examples/results/thread/diverging/",
        false
    );
}

// mod shared_counter {
//     use super::utils;
//
//     utils::generate_lola_tests_for_example_program!(
//         "./examples/programs/thread/shared_counter.rs",
//         "./examples/results/thread/shared_counter/",
//         false
//     );
// }

mod spawn_with_empty_closure {
    use super::utils;

    utils::generate_lola_tests_for_example_program!(
        "./examples/programs/thread/spawn_with_empty_closure.rs",
        "./examples/results/thread/spawn_with_empty_closure/",
        false
    );
}
