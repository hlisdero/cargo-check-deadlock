mod common;
use common::assert_output_file;

#[test]
fn minimal_deadlock_with_arc_generates_correct_dot_output_file() {
    assert_output_file(
        "./examples/programs/minimal_deadlock_with_arc.rs",
        "dot",
        "./net.dot",
        "./examples/results/minimal_deadlock_with_arc/net.dot",
    );
}

#[test]
fn minimal_deadlock_with_arc_generates_correct_lola_output_file() {
    assert_output_file(
        "./examples/programs/minimal_deadlock_with_arc.rs",
        "lola",
        "./net.lola",
        "./examples/results/minimal_deadlock_with_arc/net.lola",
    );
}

#[test]
fn minimal_deadlock_with_arc_generates_correct_pnml_output_file() {
    assert_output_file(
        "./examples/programs/minimal_deadlock_with_arc.rs",
        "pnml",
        "./net.pnml",
        "./examples/results/minimal_deadlock_with_arc/net.pnml",
    );
}
