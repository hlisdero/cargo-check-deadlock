mod common;
use common::assert_output_file;

const MINIMAL_INFINITE_LOOP_DOT_OUTPUT: &str = "\
";

const MINIMAL_INFINITE_LOOP_LOLA_OUTPUT: &str = "\
";

const MINIMAL_INFINITE_LOOP_PNML_OUTPUT: &str = "\
";

#[test]
fn minimal_infinite_loop_generates_correct_dot_output_file() {
    assert_output_file(
        "./tests/sample_programs/minimal_infinite_loop.rs",
        "dot",
        "./net.dot",
        MINIMAL_INFINITE_LOOP_DOT_OUTPUT,
    );
}

#[test]
fn minimal_infinite_loop_generates_correct_lola_output_file() {
    assert_output_file(
        "./tests/sample_programs/minimal_infinite_loop.rs",
        "lola",
        "./net.lola",
        MINIMAL_INFINITE_LOOP_LOLA_OUTPUT,
    );
}

#[test]
fn minimal_infinite_loop_generates_correct_pnml_output_file() {
    assert_output_file(
        "./tests/sample_programs/minimal_infinite_loop.rs",
        "pnml",
        "./net.pnml",
        MINIMAL_INFINITE_LOOP_PNML_OUTPUT,
    );
}
