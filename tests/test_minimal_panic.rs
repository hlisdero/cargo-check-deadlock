mod common;
use common::assert_output_file;

#[test]
fn minimal_panic_generates_correct_dot_output_file() {
    assert_output_file(
        "./tests/sample_programs/minimal_panic.rs",
        "dot",
        "./net.dot",
        "./tests/sample_results/minimal_panic/net.dot",
    );
}

#[test]
fn minimal_panic_generates_correct_lola_output_file() {
    assert_output_file(
        "./tests/sample_programs/minimal_panic.rs",
        "lola",
        "./net.lola",
        "./tests/sample_results/minimal_panic/net.lola",
    );
}

#[test]
fn minimal_panic_generates_correct_pnml_output_file() {
    assert_output_file(
        "./tests/sample_programs/minimal_panic.rs",
        "pnml",
        "./net.pnml",
        "./tests/sample_results/minimal_panic/net.pnml",
    );
}
