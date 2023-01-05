mod common;
use common::assert_output_file;

const MINIMAL_DIVERGING_FUNCTION_CALL_DOT_OUTPUT: &str = r#"digraph petrinet {
    PROGRAM_END [shape="circle" xlabel="PROGRAM_END" label=""];
    PROGRAM_PANIC [shape="circle" xlabel="PROGRAM_PANIC" label=""];
    PROGRAM_START [shape="circle" xlabel="PROGRAM_START" label="•"];
    does_not_return_DIVERGING_CALL [shape="box" xlabel="does_not_return_DIVERGING_CALL" label=""];
    PROGRAM_START -> does_not_return_DIVERGING_CALL;
}
"#;

const MINIMAL_DIVERGING_FUNCTION_CALL_LOLA_OUTPUT: &str = r#"PLACE
    PROGRAM_END,
    PROGRAM_PANIC,
    PROGRAM_START;

MARKING
    PROGRAM_START : 1;

TRANSITION does_not_return_DIVERGING_CALL
  CONSUME
    PROGRAM_START : 1;
"#;

const MINIMAL_DIVERGING_FUNCTION_CALL_PNML_OUTPUT: &str = r#"<?xml version="1.0" encoding="utf-8"?>
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
      <transition id="does_not_return_DIVERGING_CALL">
        <name>
          <text>does_not_return_DIVERGING_CALL</text>
        </name>
      </transition>
      <arc source="PROGRAM_START" target="does_not_return_DIVERGING_CALL" id="(PROGRAM_START, does_not_return_DIVERGING_CALL)">
        <name>
          <text>(PROGRAM_START, does_not_return_DIVERGING_CALL)</text>
        </name>
        <inscription>
          <text>1</text>
        </inscription>
      </arc>
    </page>
  </net>
</pnml>"#;

#[test]
fn minimal_diverging_function_call_generates_correct_dot_output_file() {
    assert_output_file(
        "./tests/sample_programs/minimal_diverging_function_call.rs",
        "dot",
        "./net.dot",
        MINIMAL_DIVERGING_FUNCTION_CALL_DOT_OUTPUT,
    );
}

#[test]
fn minimal_diverging_function_call_generates_correct_lola_output_file() {
    assert_output_file(
        "./tests/sample_programs/minimal_diverging_function_call.rs",
        "lola",
        "./net.lola",
        MINIMAL_DIVERGING_FUNCTION_CALL_LOLA_OUTPUT,
    );
}

#[test]
fn minimal_diverging_function_call_generates_correct_pnml_output_file() {
    assert_output_file(
        "./tests/sample_programs/minimal_diverging_function_call.rs",
        "pnml",
        "./net.pnml",
        MINIMAL_DIVERGING_FUNCTION_CALL_PNML_OUTPUT,
    );
}
