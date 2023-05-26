mod utils;

mod dating_philosophers {
    super::utils::generate_tests_for_example_program!(
        "./examples/programs/thread/dating_philosophers.rs",
        "./examples/results/thread/dating_philosophers/"
    );
}

mod detached {
    super::utils::generate_tests_for_example_program!(
        "./examples/programs/thread/detached.rs",
        "./examples/results/thread/detached/"
    );
}

mod dining_philosophers {
    super::utils::generate_tests_for_example_program!(
        "./examples/programs/thread/dining_philosophers.rs",
        "./examples/results/thread/dining_philosophers/"
    );
}

mod diverging {
    super::utils::generate_tests_for_example_program!(
        "./examples/programs/thread/diverging.rs",
        "./examples/results/thread/diverging/"
    );
}

mod shared_counter {
    super::utils::generate_tests_for_example_program!(
        "./examples/programs/thread/shared_counter.rs",
        "./examples/results/thread/shared_counter/"
    );
}

mod spawn_with_empty_closure {
    super::utils::generate_tests_for_example_program!(
        "./examples/programs/thread/spawn_with_empty_closure.rs",
        "./examples/results/thread/spawn_with_empty_closure/"
    );
}
