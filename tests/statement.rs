mod utils;

mod statement {
    super::utils::generate_tests_for_example_program!(
        abort,
        "./examples/programs/statement/abort.rs",
        "./examples/results/statement/abort/"
    );

    super::utils::generate_tests_for_example_program!(
        empty_main,
        "./examples/programs/statement/empty_main.rs",
        "./examples/results/statement/empty_main/"
    );

    super::utils::generate_tests_for_example_program!(
        infinite_loop,
        "./examples/programs/statement/infinite_loop.rs",
        "./examples/results/statement/infinite_loop/"
    );

    super::utils::generate_tests_for_example_program!(
        match_stmt,
        "./examples/programs/statement/match.rs",
        "./examples/results/statement/match/"
    );

    super::utils::generate_tests_for_example_program!(
        option,
        "./examples/programs/statement/option.rs",
        "./examples/results/statement/option/"
    );

    super::utils::generate_tests_for_example_program!(
        panic,
        "./examples/programs/statement/panic.rs",
        "./examples/results/statement/panic/"
    );
}

mod lola {
    super::utils::generate_lola_tests_for_example_program!(
        abort,
        "./examples/programs/statement/abort.rs",
        "./examples/results/statement/abort/",
        true
    );

    super::utils::generate_lola_tests_for_example_program!(
        empty_main,
        "./examples/programs/statement/empty_main.rs",
        "./examples/results/statement/empty_main/",
        false
    );

    super::utils::generate_lola_tests_for_example_program!(
        infinite_loop,
        "./examples/programs/statement/infinite_loop.rs",
        "./examples/results/statement/infinite_loop/",
        false
    );

    super::utils::generate_lola_tests_for_example_program!(
        match_stmt,
        "./examples/programs/statement/match.rs",
        "./examples/results/statement/match/",
        false
    );

    super::utils::generate_lola_tests_for_example_program!(
        option,
        "./examples/programs/statement/option.rs",
        "./examples/results/statement/option/",
        false
    );

    super::utils::generate_lola_tests_for_example_program!(
        panic,
        "./examples/programs/statement/panic.rs",
        "./examples/results/statement/panic/",
        false
    );
}
