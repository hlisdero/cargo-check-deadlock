mod common;
use common::assert_output_file;

#[test]
fn minimal_function_call_generates_correct_dot_output_file() {
    assert_output_file(
        "./examples/programs/minimal_function_call.rs",
        "dot",
        "./net.dot",
        "./examples/results/minimal_function_call/net.dot",
    );
}

#[test]
fn minimal_function_call_generates_correct_lola_output_file() {
    assert_output_file(
        "./examples/programs/minimal_function_call.rs",
        "lola",
        "./net.lola",
        "./examples/results/minimal_function_call/net.lola",
    );
}

#[test]
fn minimal_function_call_generates_correct_pnml_output_file() {
    assert_output_file(
        "./examples/programs/minimal_function_call.rs",
        "pnml",
        "./net.pnml",
        "./examples/results/minimal_function_call/net.pnml",
    );
}
