mod utils;

mod abort {
    use super::utils;

    utils::generate_tests_for_example_program!(
        "./examples/programs/basic/abort.rs",
        "./examples/results/basic/abort/"
    );
}

mod empty_main {
    use super::utils;

    utils::generate_tests_for_example_program!(
        "./examples/programs/basic/empty_main.rs",
        "./examples/results/basic/empty_main/"
    );
}

mod infinite_loop {
    use super::utils;

    utils::generate_tests_for_example_program!(
        "./examples/programs/basic/infinite_loop.rs",
        "./examples/results/basic/infinite_loop/"
    );
}

mod panic {
    use super::utils;

    utils::generate_tests_for_example_program!(
        "./examples/programs/basic/panic.rs",
        "./examples/results/basic/panic/"
    );
}
