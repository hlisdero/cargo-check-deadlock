mod common;
use common::assert_output_file;

#[test]
fn minimal_program_generates_correct_dot_output_file() {
    assert_output_file(
        "./examples/programs/minimal_program.rs",
        "dot",
        "./net.dot",
        "./examples/results/minimal_program/net.dot",
    );
}

#[test]
fn minimal_program_generates_correct_lola_output_file() {
    assert_output_file(
        "./examples/programs/minimal_program.rs",
        "lola",
        "./net.lola",
        "./examples/results/minimal_program/net.lola",
    );
}

#[test]
fn minimal_program_generates_correct_pnml_output_file() {
    assert_output_file(
        "./examples/programs/minimal_program.rs",
        "pnml",
        "./net.pnml",
        "./examples/results/minimal_program/net.pnml",
    );
}
