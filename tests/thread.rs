mod utils;

mod thread {
    super::utils::generate_tests_for_example_program!(
        dating_philosophers,
        "./examples/programs/thread/dating_philosophers.rs",
        "./examples/results/thread/dating_philosophers/"
    );

    super::utils::generate_tests_for_example_program!(
        detached,
        "./examples/programs/thread/detached.rs",
        "./examples/results/thread/detached/"
    );

    super::utils::generate_tests_for_example_program!(
        dining_philosophers,
        "./examples/programs/thread/dining_philosophers.rs",
        "./examples/results/thread/dining_philosophers/"
    );

    super::utils::generate_tests_for_example_program!(
        diverging,
        "./examples/programs/thread/diverging.rs",
        "./examples/results/thread/diverging/"
    );

    super::utils::generate_tests_for_example_program!(
        shared_counter,
        "./examples/programs/thread/shared_counter.rs",
        "./examples/results/thread/shared_counter/"
    );

    super::utils::generate_tests_for_example_program!(
        spawn_with_empty_closure,
        "./examples/programs/thread/spawn_with_empty_closure.rs",
        "./examples/results/thread/spawn_with_empty_closure/"
    );
}

mod lola {
    super::utils::generate_lola_tests_for_example_program!(
        dating_philosophers,
        "./examples/programs/thread/dating_philosophers.rs",
        "./examples/results/thread/dating_philosophers/",
        true
    );

    super::utils::generate_lola_tests_for_example_program!(
        detached,
        "./examples/programs/thread/detached.rs",
        "./examples/results/thread/detached/",
        false
    );

    super::utils::generate_lola_tests_for_example_program!(
        dining_philosophers,
        "./examples/programs/thread/dining_philosophers.rs",
        "./examples/results/thread/dining_philosophers/",
        true
    );

    super::utils::generate_lola_tests_for_example_program!(
        diverging,
        "./examples/programs/thread/diverging.rs",
        "./examples/results/thread/diverging/",
        false
    );

    super::utils::generate_lola_tests_for_example_program!(
        shared_counter,
        "./examples/programs/thread/shared_counter.rs",
        "./examples/results/thread/shared_counter/",
        false
    );

    super::utils::generate_lola_tests_for_example_program!(
        spawn_with_empty_closure,
        "./examples/programs/thread/spawn_with_empty_closure.rs",
        "./examples/results/thread/spawn_with_empty_closure/",
        false
    );
}
