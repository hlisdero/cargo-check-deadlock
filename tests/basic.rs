mod utils;

mod basic {
    super::utils::generate_tests_for_example_program!(
        calculator,
        "./examples/programs/basic/calculator.rs",
        "./examples/results/basic/calculator/"
    );

    super::utils::generate_tests_for_example_program!(
        greet,
        "./examples/programs/basic/greet.rs",
        "./examples/results/basic/greet/"
    );

    super::utils::generate_tests_for_example_program!(
        hello_world,
        "./examples/programs/basic/hello_world.rs",
        "./examples/results/basic/hello_world/"
    );
}

mod lola {
    super::utils::generate_lola_tests_for_example_program!(
        calculator,
        "./examples/programs/basic/calculator.rs",
        "./examples/results/basic/calculator/",
        false
    );

    super::utils::generate_lola_tests_for_example_program!(
        greet,
        "./examples/programs/basic/greet.rs",
        "./examples/results/basic/greet/",
        false
    );

    super::utils::generate_lola_tests_for_example_program!(
        hello_world,
        "./examples/programs/basic/hello_world.rs",
        "./examples/results/basic/hello_world/",
        false
    );
}
