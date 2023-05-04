mod common;
use common::assert_output_file;

#[test]
fn minimal_match_statement_generates_correct_dot_output_file() {
    assert_output_file(
        "./examples/programs/minimal_match_statement.rs",
        "dot",
        "./net.dot",
        "./examples/results/minimal_match_statement/net.dot",
    );
}

#[test]
fn minimal_match_statement_generates_correct_lola_output_file() {
    assert_output_file(
        "./examples/programs/minimal_match_statement.rs",
        "lola",
        "./net.lola",
        "./examples/results/minimal_match_statement/net.lola",
    );
}

#[test]
fn minimal_match_statement_generates_correct_pnml_output_file() {
    assert_output_file(
        "./examples/programs/minimal_match_statement.rs",
        "pnml",
        "./net.pnml",
        "./examples/results/minimal_match_statement/net.pnml",
    );
}
