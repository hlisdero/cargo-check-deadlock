mod common;
use common::assert_output_file;

#[test]
fn minimal_thread_spawn_generates_correct_dot_output_file() {
    assert_output_file(
        "./examples/programs/minimal_thread_spawn.rs",
        "dot",
        "./net.dot",
        "./examples/results/minimal_thread_spawn/net.dot",
    );
}

#[test]
fn minimal_thread_spawn_generates_correct_lola_output_file() {
    assert_output_file(
        "./examples/programs/minimal_thread_spawn.rs",
        "lola",
        "./net.lola",
        "./examples/results/minimal_thread_spawn/net.lola",
    );
}

#[test]
fn minimal_thread_spawn_generates_correct_pnml_output_file() {
    assert_output_file(
        "./examples/programs/minimal_thread_spawn.rs",
        "pnml",
        "./net.pnml",
        "./examples/results/minimal_thread_spawn/net.pnml",
    );
}
