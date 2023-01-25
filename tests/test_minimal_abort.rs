mod common;
use common::assert_output_file;

#[test]
fn minimal_abort_generates_correct_dot_output_file() {
    assert_output_file(
        "./tests/sample_programs/minimal_abort.rs",
        "dot",
        "./net.dot",
        "./tests/sample_results/minimal_abort/net.dot",
    );
}

#[test]
fn minimal_abort_generates_correct_lola_output_file() {
    assert_output_file(
        "./tests/sample_programs/minimal_abort.rs",
        "lola",
        "./net.lola",
        "./tests/sample_results/minimal_abort/net.lola",
    );
}

#[test]
fn minimal_abort_generates_correct_pnml_output_file() {
    assert_output_file(
        "./tests/sample_programs/minimal_abort.rs",
        "pnml",
        "./net.pnml",
        "./tests/sample_results/minimal_abort/net.pnml",
    );
}
