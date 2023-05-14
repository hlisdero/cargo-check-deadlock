mod utils;

mod abort {
    use super::utils;

    utils::generate_tests_for_example_program!(
        "./examples/programs/statement/abort.rs",
        "./examples/results/statement/abort/"
    );
}

mod infinite_loop {
    use super::utils;

    utils::generate_tests_for_example_program!(
        "./examples/programs/statement/infinite_loop.rs",
        "./examples/results/statement/infinite_loop/"
    );
}

mod match_stmt {
    use super::utils;

    utils::generate_tests_for_example_program!(
        "./examples/programs/statement/match.rs",
        "./examples/results/statement/match/"
    );
}

mod option {
    use super::utils;

    utils::generate_tests_for_example_program!(
        "./examples/programs/statement/option.rs",
        "./examples/results/statement/option/"
    );
}

mod panic {
    use super::utils;

    utils::generate_tests_for_example_program!(
        "./examples/programs/statement/panic.rs",
        "./examples/results/statement/panic/"
    );
}
