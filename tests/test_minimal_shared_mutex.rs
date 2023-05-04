mod common;
use common::assert_output_file;

#[test]
fn minimal_shared_mutex_generates_correct_dot_output_file() {
    assert_output_file(
        "./examples/programs/minimal_shared_mutex.rs",
        "dot",
        "./net.dot",
        "./examples/results/minimal_shared_mutex/net.dot",
    );
}

#[test]
fn minimal_shared_mutex_generates_correct_lola_output_file() {
    assert_output_file(
        "./examples/programs/minimal_shared_mutex.rs",
        "lola",
        "./net.lola",
        "./examples/results/minimal_shared_mutex/net.lola",
    );
}

#[test]
fn minimal_shared_mutex_generates_correct_pnml_output_file() {
    assert_output_file(
        "./examples/programs/minimal_shared_mutex.rs",
        "pnml",
        "./net.pnml",
        "./examples/results/minimal_shared_mutex/net.pnml",
    );
}
