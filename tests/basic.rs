mod utils;

mod abort {
    use super::utils;

    utils::generate_tests_for_example_program!(
        "./examples/programs/basic/abort.rs",
        "./examples/results/basic/abort/"
    );
}

mod calculator {
    use super::utils;

    utils::generate_tests_for_example_program!(
        "./examples/programs/basic/calculator.rs",
        "./examples/results/basic/calculator/"
    );
}

mod greet {
    use super::utils;

    utils::generate_tests_for_example_program!(
        "./examples/programs/basic/greet.rs",
        "./examples/results/basic/greet/"
    );
}

mod empty_main {
    use super::utils;

    utils::generate_tests_for_example_program!(
        "./examples/programs/basic/empty_main.rs",
        "./examples/results/basic/empty_main/"
    );
}

mod hello_world {
    use super::utils;

    utils::generate_tests_for_example_program!(
        "./examples/programs/basic/hello_world.rs",
        "./examples/results/basic/hello_world/"
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
