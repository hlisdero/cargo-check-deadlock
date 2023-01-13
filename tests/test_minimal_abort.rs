mod common;
use common::assert_output_file;

const MINIMAL_ABORT_DOT_OUTPUT: &str = r#"digraph petrinet {
    PROGRAM_END [shape="circle" xlabel="PROGRAM_END" label=""];
    PROGRAM_PANIC [shape="circle" xlabel="PROGRAM_PANIC" label=""];
    PROGRAM_START [shape="circle" xlabel="PROGRAM_START" label="•"];
    std_process_abort_DIVERGING_CALL [shape="box" xlabel="std_process_abort_DIVERGING_CALL" label=""];
    PROGRAM_START -> std_process_abort_DIVERGING_CALL;
}
"#;

const MINIMAL_ABORT_LOLA_OUTPUT: &str = r#"PLACE
    PROGRAM_END,
    PROGRAM_PANIC,
    PROGRAM_START;

MARKING
    PROGRAM_START : 1;

TRANSITION std_process_abort_DIVERGING_CALL
  CONSUME
    PROGRAM_START : 1;
"#;

const MINIMAL_ABORT_PNML_OUTPUT: &str = r#"<?xml version="1.0" encoding="utf-8"?>
<pnml xmlns="http://www.pnml.org/version-2009/grammar/pnml">
  <net id="net0" type="http://www.pnml.org/version-2009/grammar/ptnet">
    <page id="page0">
      <place id="PROGRAM_END">
        <name>
          <text>PROGRAM_END</text>
        </name>
      </place>
      <place id="PROGRAM_PANIC">
        <name>
          <text>PROGRAM_PANIC</text>
        </name>
      </place>
      <place id="PROGRAM_START">
        <name>
          <text>PROGRAM_START</text>
        </name>
        <initialMarking>
          <text>1</text>
        </initialMarking>
      </place>
      <transition id="std_process_abort_DIVERGING_CALL">
        <name>
          <text>std_process_abort_DIVERGING_CALL</text>
        </name>
      </transition>
      <arc source="PROGRAM_START" target="std_process_abort_DIVERGING_CALL" id="(PROGRAM_START, std_process_abort_DIVERGING_CALL)">
        <name>
          <text>(PROGRAM_START, std_process_abort_DIVERGING_CALL)</text>
        </name>
        <inscription>
          <text>1</text>
        </inscription>
      </arc>
    </page>
  </net>
</pnml>"#;

#[test]
fn minimal_abort_generates_correct_dot_output_file() {
    assert_output_file(
        "./tests/sample_programs/minimal_abort.rs",
        "dot",
        "./net.dot",
        MINIMAL_ABORT_DOT_OUTPUT,
    );
}

#[test]
fn minimal_abort_generates_correct_lola_output_file() {
    assert_output_file(
        "./tests/sample_programs/minimal_abort.rs",
        "lola",
        "./net.lola",
        MINIMAL_ABORT_LOLA_OUTPUT,
    );
}

#[test]
fn minimal_abort_generates_correct_pnml_output_file() {
    assert_output_file(
        "./tests/sample_programs/minimal_abort.rs",
        "pnml",
        "./net.pnml",
        MINIMAL_ABORT_PNML_OUTPUT,
    );
}
