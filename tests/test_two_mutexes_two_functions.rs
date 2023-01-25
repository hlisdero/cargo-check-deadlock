mod common;
use common::assert_output_file;

#[test]
fn two_mutexes_two_functions_generates_correct_dot_output_file() {
    assert_output_file(
        "./tests/sample_programs/two_mutexes_two_functions.rs",
        "dot",
        "./net.dot",
        "./tests/sample_results/two_mutexes_two_functions/net.dot",
    );
}

#[test]
fn two_mutexes_two_functions_generates_correct_lola_output_file() {
    assert_output_file(
        "./tests/sample_programs/two_mutexes_two_functions.rs",
        "lola",
        "./net.lola",
        "./tests/sample_results/two_mutexes_two_functions/net.lola",
    );
}

#[test]
fn two_mutexes_two_functions_generates_correct_pnml_output_file() {
    assert_output_file(
        "./tests/sample_programs/two_mutexes_two_functions.rs",
        "pnml",
        "./net.pnml",
        "./tests/sample_results/two_mutexes_two_functions/net.pnml",
    );
}
