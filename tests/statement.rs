mod utils;

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
