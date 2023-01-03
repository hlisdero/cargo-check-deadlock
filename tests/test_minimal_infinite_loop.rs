mod common;
use common::assert_output_file;

const MINIMAL_INFINITE_LOOP_DOT_OUTPUT: &str = r#"digraph petrinet {
    PROGRAM_END [shape="circle" xlabel="PROGRAM_END" label=""];
    PROGRAM_PANIC [shape="circle" xlabel="PROGRAM_PANIC" label=""];
    PROGRAM_START [shape="circle" xlabel="PROGRAM_START" label="•"];
    main_BASIC_BLOCK_1 [shape="circle" xlabel="main_BASIC_BLOCK_1" label=""];
    main_BASIC_BLOCK_END_PLACE_0 [shape="circle" xlabel="main_BASIC_BLOCK_END_PLACE_0" label=""];
    main_BASIC_BLOCK_END_PLACE_1 [shape="circle" xlabel="main_BASIC_BLOCK_END_PLACE_1" label=""];
    main_BASIC_BLOCK_EMPTY_0 [shape="box" xlabel="main_BASIC_BLOCK_EMPTY_0" label=""];
    main_BASIC_BLOCK_EMPTY_1 [shape="box" xlabel="main_BASIC_BLOCK_EMPTY_1" label=""];
    main_GOTO_0 [shape="box" xlabel="main_GOTO_0" label=""];
    main_GOTO_1 [shape="box" xlabel="main_GOTO_1" label=""];
    PROGRAM_START -> main_BASIC_BLOCK_EMPTY_0;
    main_BASIC_BLOCK_1 -> main_BASIC_BLOCK_EMPTY_1;
    main_BASIC_BLOCK_END_PLACE_0 -> main_GOTO_0;
    main_BASIC_BLOCK_END_PLACE_1 -> main_GOTO_1;
    main_BASIC_BLOCK_EMPTY_0 -> main_BASIC_BLOCK_END_PLACE_0;
    main_BASIC_BLOCK_EMPTY_1 -> main_BASIC_BLOCK_END_PLACE_1;
    main_GOTO_0 -> main_BASIC_BLOCK_1;
    main_GOTO_1 -> main_BASIC_BLOCK_1;
}
"#;

const MINIMAL_INFINITE_LOOP_LOLA_OUTPUT: &str = r#"PLACE
    PROGRAM_END,
    PROGRAM_PANIC,
    PROGRAM_START,
    main_BASIC_BLOCK_1,
    main_BASIC_BLOCK_END_PLACE_0,
    main_BASIC_BLOCK_END_PLACE_1;

MARKING
    PROGRAM_START : 1,
TRANSITION main_BASIC_BLOCK_EMPTY_0
  CONSUME
    PROGRAM_START : 1;
  PRODUCE
    main_BASIC_BLOCK_END_PLACE_0 : 1;
TRANSITION main_BASIC_BLOCK_EMPTY_1
  CONSUME
    main_BASIC_BLOCK_1 : 1;
  PRODUCE
    main_BASIC_BLOCK_END_PLACE_1 : 1;
TRANSITION main_GOTO_0
  CONSUME
    main_BASIC_BLOCK_END_PLACE_0 : 1;
  PRODUCE
    main_BASIC_BLOCK_1 : 1;
TRANSITION main_GOTO_1
  CONSUME
    main_BASIC_BLOCK_END_PLACE_1 : 1;
  PRODUCE
    main_BASIC_BLOCK_1 : 1;
"#;

const MINIMAL_INFINITE_LOOP_PNML_OUTPUT: &str = r#"<?xml version="1.0" encoding="utf-8"?>
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
      <place id="main_BASIC_BLOCK_1">
        <name>
          <text>main_BASIC_BLOCK_1</text>
        </name>
      </place>
      <place id="main_BASIC_BLOCK_END_PLACE_0">
        <name>
          <text>main_BASIC_BLOCK_END_PLACE_0</text>
        </name>
      </place>
      <place id="main_BASIC_BLOCK_END_PLACE_1">
        <name>
          <text>main_BASIC_BLOCK_END_PLACE_1</text>
        </name>
      </place>
      <transition id="main_BASIC_BLOCK_EMPTY_0">
        <name>
          <text>main_BASIC_BLOCK_EMPTY_0</text>
        </name>
      </transition>
      <transition id="main_BASIC_BLOCK_EMPTY_1">
        <name>
          <text>main_BASIC_BLOCK_EMPTY_1</text>
        </name>
      </transition>
      <transition id="main_GOTO_0">
        <name>
          <text>main_GOTO_0</text>
        </name>
      </transition>
      <transition id="main_GOTO_1">
        <name>
          <text>main_GOTO_1</text>
        </name>
      </transition>
      <arc source="PROGRAM_START" target="main_BASIC_BLOCK_EMPTY_0" id="(PROGRAM_START, main_BASIC_BLOCK_EMPTY_0)">
        <name>
          <text>(PROGRAM_START, main_BASIC_BLOCK_EMPTY_0)</text>
        </name>
        <inscription>
          <text>1</text>
        </inscription>
      </arc>
      <arc source="main_BASIC_BLOCK_1" target="main_BASIC_BLOCK_EMPTY_1" id="(main_BASIC_BLOCK_1, main_BASIC_BLOCK_EMPTY_1)">
        <name>
          <text>(main_BASIC_BLOCK_1, main_BASIC_BLOCK_EMPTY_1)</text>
        </name>
        <inscription>
          <text>1</text>
        </inscription>
      </arc>
      <arc source="main_BASIC_BLOCK_END_PLACE_0" target="main_GOTO_0" id="(main_BASIC_BLOCK_END_PLACE_0, main_GOTO_0)">
        <name>
          <text>(main_BASIC_BLOCK_END_PLACE_0, main_GOTO_0)</text>
        </name>
        <inscription>
          <text>1</text>
        </inscription>
      </arc>
      <arc source="main_BASIC_BLOCK_END_PLACE_1" target="main_GOTO_1" id="(main_BASIC_BLOCK_END_PLACE_1, main_GOTO_1)">
        <name>
          <text>(main_BASIC_BLOCK_END_PLACE_1, main_GOTO_1)</text>
        </name>
        <inscription>
          <text>1</text>
        </inscription>
      </arc>
      <arc source="main_BASIC_BLOCK_EMPTY_0" target="main_BASIC_BLOCK_END_PLACE_0" id="(main_BASIC_BLOCK_EMPTY_0, main_BASIC_BLOCK_END_PLACE_0)">
        <name>
          <text>(main_BASIC_BLOCK_EMPTY_0, main_BASIC_BLOCK_END_PLACE_0)</text>
        </name>
        <inscription>
          <text>1</text>
        </inscription>
      </arc>
      <arc source="main_BASIC_BLOCK_EMPTY_1" target="main_BASIC_BLOCK_END_PLACE_1" id="(main_BASIC_BLOCK_EMPTY_1, main_BASIC_BLOCK_END_PLACE_1)">
        <name>
          <text>(main_BASIC_BLOCK_EMPTY_1, main_BASIC_BLOCK_END_PLACE_1)</text>
        </name>
        <inscription>
          <text>1</text>
        </inscription>
      </arc>
      <arc source="main_GOTO_0" target="main_BASIC_BLOCK_1" id="(main_GOTO_0, main_BASIC_BLOCK_1)">
        <name>
          <text>(main_GOTO_0, main_BASIC_BLOCK_1)</text>
        </name>
        <inscription>
          <text>1</text>
        </inscription>
      </arc>
      <arc source="main_GOTO_1" target="main_BASIC_BLOCK_1" id="(main_GOTO_1, main_BASIC_BLOCK_1)">
        <name>
          <text>(main_GOTO_1, main_BASIC_BLOCK_1)</text>
        </name>
        <inscription>
          <text>1</text>
        </inscription>
      </arc>
    </page>
  </net>
</pnml>"#;

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
