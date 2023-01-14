mod common;
use common::assert_output_file;

const MINIMAL_PANIC_DOT_OUTPUT: &str = r#"digraph petrinet {
    PROGRAM_END [shape="circle" xlabel="PROGRAM_END" label=""];
    PROGRAM_PANIC [shape="circle" xlabel="PROGRAM_PANIC" label=""];
    PROGRAM_START [shape="circle" xlabel="PROGRAM_START" label="•"];
    main_PANIC [shape="box" xlabel="" label="main_PANIC"];
    PROGRAM_START -> main_PANIC;
    main_PANIC -> PROGRAM_PANIC;
}
"#;

const MINIMAL_PANIC_LOLA_OUTPUT: &str = r#"PLACE
    PROGRAM_END,
    PROGRAM_PANIC,
    PROGRAM_START;

MARKING
    PROGRAM_START : 1;

TRANSITION main_PANIC
  CONSUME
    PROGRAM_START : 1;
  PRODUCE
    PROGRAM_PANIC : 1;
"#;

const MINIMAL_PANIC_PNML_OUTPUT: &str = r#"<?xml version="1.0" encoding="utf-8"?>
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
      <transition id="main_PANIC">
        <name>
          <text>main_PANIC</text>
        </name>
      </transition>
      <arc source="PROGRAM_START" target="main_PANIC" id="(PROGRAM_START, main_PANIC)">
        <name>
          <text>(PROGRAM_START, main_PANIC)</text>
        </name>
        <inscription>
          <text>1</text>
        </inscription>
      </arc>
      <arc source="main_PANIC" target="PROGRAM_PANIC" id="(main_PANIC, PROGRAM_PANIC)">
        <name>
          <text>(main_PANIC, PROGRAM_PANIC)</text>
        </name>
        <inscription>
          <text>1</text>
        </inscription>
      </arc>
    </page>
  </net>
</pnml>"#;

#[test]
fn minimal_panic_generates_correct_dot_output_file() {
    assert_output_file(
        "./tests/sample_programs/minimal_panic.rs",
        "dot",
        "./net.dot",
        MINIMAL_PANIC_DOT_OUTPUT,
    );
}

#[test]
fn minimal_panic_generates_correct_lola_output_file() {
    assert_output_file(
        "./tests/sample_programs/minimal_panic.rs",
        "lola",
        "./net.lola",
        MINIMAL_PANIC_LOLA_OUTPUT,
    );
}

#[test]
fn minimal_panic_generates_correct_pnml_output_file() {
    assert_output_file(
        "./tests/sample_programs/minimal_panic.rs",
        "pnml",
        "./net.pnml",
        MINIMAL_PANIC_PNML_OUTPUT,
    );
}
