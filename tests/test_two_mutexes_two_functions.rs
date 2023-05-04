mod common;
use common::assert_output_file;

#[test]
fn two_mutexes_two_functions_generates_correct_dot_output_file() {
    assert_output_file(
        "./examples/programs/two_mutexes_two_functions.rs",
        "dot",
        "./net.dot",
        "./examples/results/two_mutexes_two_functions/net.dot",
    );
}

#[test]
fn two_mutexes_two_functions_generates_correct_lola_output_file() {
    assert_output_file(
        "./examples/programs/two_mutexes_two_functions.rs",
        "lola",
        "./net.lola",
        "./examples/results/two_mutexes_two_functions/net.lola",
    );
}

#[test]
fn two_mutexes_two_functions_generates_correct_pnml_output_file() {
    assert_output_file(
        "./examples/programs/two_mutexes_two_functions.rs",
        "pnml",
        "./net.pnml",
        "./examples/results/two_mutexes_two_functions/net.pnml",
    );
}
