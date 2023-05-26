//! Tests for the model checker `LoLA`
//!
//! Check that the result matches the expected output for every program.

mod utils;

mod calculator {
    super::utils::generate_lola_tests_for_example_program!(
        "./examples/programs/basic/calculator.rs",
        "./examples/results/basic/calculator/",
        false
    );
}

mod greet {
    super::utils::generate_lola_tests_for_example_program!(
        "./examples/programs/basic/greet.rs",
        "./examples/results/basic/greet/",
        false
    );
}

mod hello_world {
    super::utils::generate_lola_tests_for_example_program!(
        "./examples/programs/basic/hello_world.rs",
        "./examples/results/basic/hello_world/",
        false
    );
}

mod infinite_wait_deadlock {
    super::utils::generate_lola_tests_for_example_program!(
        "./examples/programs/condvar/infinite_wait_deadlock.rs",
        "./examples/results/condvar/infinite_wait_deadlock/",
        true
    );
}

mod producer_consumer {
    super::utils::generate_lola_tests_for_example_program!(
        "./examples/programs/condvar/producer_consumer.rs",
        "./examples/results/condvar/producer_consumer/",
        false
    );
}

mod self_notify_lost_signal {
    super::utils::generate_lola_tests_for_example_program!(
        "./examples/programs/condvar/self_notify_lost_signal.rs",
        "./examples/results/condvar/self_notify_lost_signal/",
        true
    );
}

mod set_condition_before_wait {
    super::utils::generate_lola_tests_for_example_program!(
        "./examples/programs/condvar/set_condition_before_wait.rs",
        "./examples/results/condvar/set_condition_before_wait/",
        false
    );
}

mod wait {
    super::utils::generate_lola_tests_for_example_program!(
        "./examples/programs/condvar/wait.rs",
        "./examples/results/condvar/wait/",
        false
    );
}

mod wait_while {
    super::utils::generate_lola_tests_for_example_program!(
        "./examples/programs/condvar/wait_while.rs",
        "./examples/results/condvar/wait_while/",
        false
    );
}

mod diverging {
    super::utils::generate_lola_tests_for_example_program!(
        "./examples/programs/function_call/diverging.rs",
        "./examples/results/function_call/diverging/",
        true
    );
}

mod find_even {
    super::utils::generate_lola_tests_for_example_program!(
        "./examples/programs/function_call/find_even.rs",
        "./examples/results/function_call/find_even/",
        false
    );
}

mod empty_function {
    super::utils::generate_lola_tests_for_example_program!(
        "./examples/programs/function_call/empty_function.rs",
        "./examples/results/function_call/empty_function/",
        false
    );
}

mod in_a_loop {
    super::utils::generate_lola_tests_for_example_program!(
        "./examples/programs/function_call/in_a_loop.rs",
        "./examples/results/function_call/in_a_loop/",
        false
    );
}

mod two_calls_same_function {
    super::utils::generate_lola_tests_for_example_program!(
        "./examples/programs/function_call/two_calls_same_function.rs",
        "./examples/results/function_call/two_calls_same_function/",
        false
    );
}

mod double_lock_deadlock_in_function {
    super::utils::generate_lola_tests_for_example_program!(
        "./examples/programs/mutex/double_lock_deadlock_in_function.rs",
        "./examples/results/mutex/double_lock_deadlock_in_function/",
        true
    );
}

mod double_lock_deadlock_with_arc {
    super::utils::generate_lola_tests_for_example_program!(
        "./examples/programs/mutex/double_lock_deadlock_with_arc.rs",
        "./examples/results/mutex/double_lock_deadlock_with_arc/",
        true
    );
}

mod double_lock_deadlock {
    super::utils::generate_lola_tests_for_example_program!(
        "./examples/programs/mutex/double_lock_deadlock.rs",
        "./examples/results/mutex/double_lock_deadlock/",
        true
    );
}

mod drop_mutex_guard_in_arc_manually {
    super::utils::generate_lola_tests_for_example_program!(
        "./examples/programs/mutex/drop_mutex_guard_in_arc_manually.rs",
        "./examples/results/mutex/drop_mutex_guard_in_arc_manually/",
        false
    );
}

mod drop_mutex_guard_manually {
    super::utils::generate_lola_tests_for_example_program!(
        "./examples/programs/mutex/drop_mutex_guard_manually.rs",
        "./examples/results/mutex/drop_mutex_guard_manually/",
        false
    );
}

mod two_threads_sharing_mutex {
    super::utils::generate_lola_tests_for_example_program!(
        "./examples/programs/mutex/two_threads_sharing_mutex.rs",
        "./examples/results/mutex/two_threads_sharing_mutex/",
        false
    );
}

mod abort {
    super::utils::generate_lola_tests_for_example_program!(
        "./examples/programs/statement/abort.rs",
        "./examples/results/statement/abort/",
        true
    );
}

mod empty_main {
    super::utils::generate_lola_tests_for_example_program!(
        "./examples/programs/statement/empty_main.rs",
        "./examples/results/statement/empty_main/",
        false
    );
}

mod infinite_loop {
    super::utils::generate_lola_tests_for_example_program!(
        "./examples/programs/statement/infinite_loop.rs",
        "./examples/results/statement/infinite_loop/",
        false
    );
}

mod match_stmt {
    super::utils::generate_lola_tests_for_example_program!(
        "./examples/programs/statement/match.rs",
        "./examples/results/statement/match/",
        false
    );
}

mod option {
    super::utils::generate_lola_tests_for_example_program!(
        "./examples/programs/statement/option.rs",
        "./examples/results/statement/option/",
        false
    );
}

mod panic {
    super::utils::generate_lola_tests_for_example_program!(
        "./examples/programs/statement/panic.rs",
        "./examples/results/statement/panic/",
        false
    );
}

mod dating_philosophers {
    super::utils::generate_lola_tests_for_example_program!(
        "./examples/programs/thread/dating_philosophers.rs",
        "./examples/results/thread/dating_philosophers/",
        true
    );
}

mod detached {
    super::utils::generate_lola_tests_for_example_program!(
        "./examples/programs/thread/detached.rs",
        "./examples/results/thread/detached/",
        false
    );
}

mod dining_philosophers {
    super::utils::generate_lola_tests_for_example_program!(
        "./examples/programs/thread/dining_philosophers.rs",
        "./examples/results/thread/dining_philosophers/",
        true
    );
}

mod diverging_thread {
    super::utils::generate_lola_tests_for_example_program!(
        "./examples/programs/thread/diverging.rs",
        "./examples/results/thread/diverging/",
        false
    );
}

mod shared_counter {
    super::utils::generate_lola_tests_for_example_program!(
        "./examples/programs/thread/shared_counter.rs",
        "./examples/results/thread/shared_counter/",
        false
    );
}

mod spawn_with_empty_closure {
    super::utils::generate_lola_tests_for_example_program!(
        "./examples/programs/thread/spawn_with_empty_closure.rs",
        "./examples/results/thread/spawn_with_empty_closure/",
        false
    );
}
