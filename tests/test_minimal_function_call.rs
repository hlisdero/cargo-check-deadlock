mod common;
use common::assert_output_file;

const MINIMAL_FUNCTION_CALL_DOT_OUTPUT: &str = r#"digraph petrinet {
    PROGRAM_END [shape="circle" xlabel="PROGRAM_END" label=""];
    PROGRAM_PANIC [shape="circle" xlabel="PROGRAM_PANIC" label=""];
    PROGRAM_START [shape="circle" xlabel="PROGRAM_START" label="•"];
    main_BB1 [shape="circle" xlabel="main_BB1" label=""];
    call_RETURN [shape="box" xlabel="" label="call_RETURN"];
    main_RETURN [shape="box" xlabel="" label="main_RETURN"];
    PROGRAM_START -> call_RETURN;
    main_BB1 -> main_RETURN;
    call_RETURN -> main_BB1;
    main_RETURN -> PROGRAM_END;
}
"#;

const MINIMAL_FUNCTION_CALL_LOLA_OUTPUT: &str = r#"PLACE
    PROGRAM_END,
    PROGRAM_PANIC,
    PROGRAM_START,
    main_BB1;

MARKING
    PROGRAM_START : 1,
TRANSITION call_RETURN
  CONSUME
    PROGRAM_START : 1;
  PRODUCE
    main_BB1 : 1;
TRANSITION main_RETURN
  CONSUME
    main_BB1 : 1;
  PRODUCE
    PROGRAM_END : 1;
"#;

const MINIMAL_FUNCTION_CALL_PNML_OUTPUT: &str = r#"<?xml version="1.0" encoding="utf-8"?>
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
      <place id="main_BB1">
        <name>
          <text>main_BB1</text>
        </name>
      </place>
      <transition id="call_RETURN">
        <name>
          <text>call_RETURN</text>
        </name>
      </transition>
      <transition id="main_RETURN">
        <name>
          <text>main_RETURN</text>
        </name>
      </transition>
      <arc source="PROGRAM_START" target="call_RETURN" id="(PROGRAM_START, call_RETURN)">
        <name>
          <text>(PROGRAM_START, call_RETURN)</text>
        </name>
        <inscription>
          <text>1</text>
        </inscription>
      </arc>
      <arc source="main_BB1" target="main_RETURN" id="(main_BB1, main_RETURN)">
        <name>
          <text>(main_BB1, main_RETURN)</text>
        </name>
        <inscription>
          <text>1</text>
        </inscription>
      </arc>
      <arc source="call_RETURN" target="main_BB1" id="(call_RETURN, main_BB1)">
        <name>
          <text>(call_RETURN, main_BB1)</text>
        </name>
        <inscription>
          <text>1</text>
        </inscription>
      </arc>
      <arc source="main_RETURN" target="PROGRAM_END" id="(main_RETURN, PROGRAM_END)">
        <name>
          <text>(main_RETURN, PROGRAM_END)</text>
        </name>
        <inscription>
          <text>1</text>
        </inscription>
      </arc>
    </page>
  </net>
</pnml>"#;

#[test]
fn minimal_function_call_generates_correct_dot_output_file() {
    assert_output_file(
        "./tests/sample_programs/minimal_function_call.rs",
        "dot",
        "./net.dot",
        MINIMAL_FUNCTION_CALL_DOT_OUTPUT,
    );
}

#[test]
fn minimal_function_call_generates_correct_lola_output_file() {
    assert_output_file(
        "./tests/sample_programs/minimal_function_call.rs",
        "lola",
        "./net.lola",
        MINIMAL_FUNCTION_CALL_LOLA_OUTPUT,
    );
}

#[test]
fn minimal_function_call_generates_correct_pnml_output_file() {
    assert_output_file(
        "./tests/sample_programs/minimal_function_call.rs",
        "pnml",
        "./net.pnml",
        MINIMAL_FUNCTION_CALL_PNML_OUTPUT,
    );
}
