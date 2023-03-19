mod common;
use common::assert_output_file;

#[test]
fn minimal_shared_condvar_generates_correct_dot_output_file() {
    assert_output_file(
        "./tests/sample_programs/minimal_shared_condvar.rs",
        "dot",
        "./net.dot",
        "./tests/sample_results/minimal_shared_condvar/net.dot",
    );
}

#[test]
fn minimal_shared_condvar_generates_correct_lola_output_file() {
    assert_output_file(
        "./tests/sample_programs/minimal_shared_condvar.rs",
        "lola",
        "./net.lola",
        "./tests/sample_results/minimal_shared_condvar/net.lola",
    );
}

#[test]
fn minimal_shared_condvar_generates_correct_pnml_output_file() {
    assert_output_file(
        "./tests/sample_programs/minimal_shared_condvar.rs",
        "pnml",
        "./net.pnml",
        "./tests/sample_results/minimal_shared_condvar/net.pnml",
    );
}