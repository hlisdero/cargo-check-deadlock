mod utils;

mod detached {
    use super::utils;

    utils::generate_tests_for_example_program!(
        "./examples/programs/thread/detached.rs",
        "./examples/results/thread/detached/"
    );
}

mod diverging {
    use super::utils;

    utils::generate_tests_for_example_program!(
        "./examples/programs/thread/diverging.rs",
        "./examples/results/thread/diverging/"
    );
}

mod spawn_with_empty_closure {
    use super::utils;

    utils::generate_tests_for_example_program!(
        "./examples/programs/thread/spawn_with_empty_closure.rs",
        "./examples/results/thread/spawn_with_empty_closure/"
    );
}
