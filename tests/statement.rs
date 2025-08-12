mod utils;

mod abort {
    super::utils::generate_tests_for_example_program!(
        abort,
        "./examples/programs/statement/abort.rs",
        "./examples/results/statement/abort/"
    );
}

mod empty_main {
    super::utils::generate_tests_for_example_program!(
        empty_main,
        "./examples/programs/statement/empty_main.rs",
        "./examples/results/statement/empty_main/"
    );
}

mod infinite_loop {
    super::utils::generate_tests_for_example_program!(
        infinite_loop,
        "./examples/programs/statement/infinite_loop.rs",
        "./examples/results/statement/infinite_loop/"
    );
}

mod match_stmt {
    super::utils::generate_tests_for_example_program!(
        match_stmt,
        "./examples/programs/statement/match.rs",
        "./examples/results/statement/match/"
    );
}

mod option {
    super::utils::generate_tests_for_example_program!(
        option,
        "./examples/programs/statement/option.rs",
        "./examples/results/statement/option/"
    );
}

mod panic {
    super::utils::generate_tests_for_example_program!(
        panic,
        "./examples/programs/statement/panic.rs",
        "./examples/results/statement/panic/"
    );
}
