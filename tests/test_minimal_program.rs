mod common;
use common::assert_output_file;

#[test]
fn minimal_program_generates_correct_dot_output_file() {
    assert_output_file(
        "./tests/sample_programs/minimal_program.rs",
        "dot",
        "./net.dot",
        "./tests/sample_results/minimal_program/net.dot",
    );
}

#[test]
fn minimal_program_generates_correct_lola_output_file() {
    assert_output_file(
        "./tests/sample_programs/minimal_program.rs",
        "lola",
        "./net.lola",
        "./tests/sample_results/minimal_program/net.lola",
    );
}

#[test]
fn minimal_program_generates_correct_pnml_output_file() {
    assert_output_file(
        "./tests/sample_programs/minimal_program.rs",
        "pnml",
        "./net.pnml",
        "./tests/sample_results/minimal_program/net.pnml",
    );
}
