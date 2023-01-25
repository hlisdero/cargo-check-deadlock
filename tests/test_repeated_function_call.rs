mod common;
use common::assert_output_file;

#[test]
fn repeated_function_call_generates_correct_dot_output_file() {
    assert_output_file(
        "./tests/sample_programs/repeated_function_call.rs",
        "dot",
        "./net.dot",
        "./tests/sample_results/repeated_function_call/net.dot",
    );
}

#[test]
fn repeated_function_call_generates_correct_lola_output_file() {
    assert_output_file(
        "./tests/sample_programs/repeated_function_call.rs",
        "lola",
        "./net.lola",
        "./tests/sample_results/repeated_function_call/net.lola",
    );
}

#[test]
fn repeated_function_call_generates_correct_pnml_output_file() {
    assert_output_file(
        "./tests/sample_programs/repeated_function_call.rs",
        "pnml",
        "./net.pnml",
        "./tests/sample_results/repeated_function_call/net.pnml",
    );
}
