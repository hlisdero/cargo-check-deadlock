mod common;
use common::assert_output_file;

#[test]
fn minimal_condvar_notify_generates_correct_dot_output_file() {
    assert_output_file(
        "./tests/sample_programs/minimal_condvar_notify.rs",
        "dot",
        "./net.dot",
        "./tests/sample_results/minimal_condvar_notify/net.dot",
    );
}

#[test]
fn minimal_condvar_notify_generates_correct_lola_output_file() {
    assert_output_file(
        "./tests/sample_programs/minimal_condvar_notify.rs",
        "lola",
        "./net.lola",
        "./tests/sample_results/minimal_condvar_notify/net.lola",
    );
}

#[test]
fn minimal_condvar_notify_generates_correct_pnml_output_file() {
    assert_output_file(
        "./tests/sample_programs/minimal_condvar_notify.rs",
        "pnml",
        "./net.pnml",
        "./tests/sample_results/minimal_condvar_notify/net.pnml",
    );
}
