mod common;
use common::assert_output_file;

const MINIMAL_MATCH_STATEMENT_DOT_OUTPUT: &str = r#"digraph petrinet {
    PROGRAM_END [shape="circle" xlabel="PROGRAM_END" label=""];
    PROGRAM_PANIC [shape="circle" xlabel="PROGRAM_PANIC" label=""];
    PROGRAM_START [shape="circle" xlabel="PROGRAM_START" label="•"];
    main_BB1 [shape="circle" xlabel="main_BB1" label=""];
    main_BB1_END_PLACE [shape="circle" xlabel="main_BB1_END_PLACE" label=""];
    main_BB2 [shape="circle" xlabel="main_BB2" label=""];
    main_BB3 [shape="circle" xlabel="main_BB3" label=""];
    main_BB4 [shape="circle" xlabel="main_BB4" label=""];
    main_BB5 [shape="circle" xlabel="main_BB5" label=""];
    main_BB1_STMT0 [shape="box" xlabel="" label="main_BB1_STMT0"];
    main_DROP_2 [shape="box" xlabel="" label="main_DROP_2"];
    main_DROP_4 [shape="box" xlabel="" label="main_DROP_4"];
    main_RETURN [shape="box" xlabel="" label="main_RETURN"];
    main_UNWIND_5 [shape="box" xlabel="" label="main_UNWIND_5"];
    std_env_args_CALL [shape="box" xlabel="" label="std_env_args_CALL"];
    std_iter_ExactSizeIterator_len_CALL [shape="box" xlabel="" label="std_iter_ExactSizeIterator_len_CALL"];
    std_iter_ExactSizeIterator_len_CALL_UNWIND [shape="box" xlabel="" label="std_iter_ExactSizeIterator_len_CALL_UNWIND"];
    PROGRAM_START -> std_env_args_CALL;
    main_BB1 -> main_BB1_STMT0;
    main_BB1_END_PLACE -> std_iter_ExactSizeIterator_len_CALL;
    main_BB1_END_PLACE -> std_iter_ExactSizeIterator_len_CALL_UNWIND;
    main_BB2 -> main_DROP_2;
    main_BB3 -> main_RETURN;
    main_BB4 -> main_DROP_4;
    main_BB5 -> main_UNWIND_5;
    main_BB1_STMT0 -> main_BB1_END_PLACE;
    main_DROP_2 -> main_BB3;
    main_DROP_4 -> main_BB5;
    main_RETURN -> PROGRAM_END;
    main_UNWIND_5 -> PROGRAM_PANIC;
    std_env_args_CALL -> main_BB1;
    std_iter_ExactSizeIterator_len_CALL -> main_BB2;
    std_iter_ExactSizeIterator_len_CALL_UNWIND -> main_BB4;
}
"#;

const MINIMAL_MATCH_STATEMENT_LOLA_OUTPUT: &str = r#"PLACE
    PROGRAM_END,
    PROGRAM_PANIC,
    PROGRAM_START,
    main_BB1,
    main_BB1_END_PLACE,
    main_BB2,
    main_BB3,
    main_BB4,
    main_BB5;

MARKING
    PROGRAM_START : 1,
TRANSITION main_BB1_STMT0
  CONSUME
    main_BB1 : 1;
  PRODUCE
    main_BB1_END_PLACE : 1;
TRANSITION main_DROP_2
  CONSUME
    main_BB2 : 1;
  PRODUCE
    main_BB3 : 1;
TRANSITION main_DROP_4
  CONSUME
    main_BB4 : 1;
  PRODUCE
    main_BB5 : 1;
TRANSITION main_RETURN
  CONSUME
    main_BB3 : 1;
  PRODUCE
    PROGRAM_END : 1;
TRANSITION main_UNWIND_5
  CONSUME
    main_BB5 : 1;
  PRODUCE
    PROGRAM_PANIC : 1;
TRANSITION std_env_args_CALL
  CONSUME
    PROGRAM_START : 1;
  PRODUCE
    main_BB1 : 1;
TRANSITION std_iter_ExactSizeIterator_len_CALL
  CONSUME
    main_BB1_END_PLACE : 1;
  PRODUCE
    main_BB2 : 1;
TRANSITION std_iter_ExactSizeIterator_len_CALL_UNWIND
  CONSUME
    main_BB1_END_PLACE : 1;
  PRODUCE
    main_BB4 : 1;
"#;

const MINIMAL_MATCH_STATEMENT_PNML_OUTPUT: &str = r#"<?xml version="1.0" encoding="utf-8"?>
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
      <place id="main_BB1_END_PLACE">
        <name>
          <text>main_BB1_END_PLACE</text>
        </name>
      </place>
      <place id="main_BB2">
        <name>
          <text>main_BB2</text>
        </name>
      </place>
      <place id="main_BB3">
        <name>
          <text>main_BB3</text>
        </name>
      </place>
      <place id="main_BB4">
        <name>
          <text>main_BB4</text>
        </name>
      </place>
      <place id="main_BB5">
        <name>
          <text>main_BB5</text>
        </name>
      </place>
      <transition id="main_BB1_STMT0">
        <name>
          <text>main_BB1_STMT0</text>
        </name>
      </transition>
      <transition id="main_DROP_2">
        <name>
          <text>main_DROP_2</text>
        </name>
      </transition>
      <transition id="main_DROP_4">
        <name>
          <text>main_DROP_4</text>
        </name>
      </transition>
      <transition id="main_RETURN">
        <name>
          <text>main_RETURN</text>
        </name>
      </transition>
      <transition id="main_UNWIND_5">
        <name>
          <text>main_UNWIND_5</text>
        </name>
      </transition>
      <transition id="std_env_args_CALL">
        <name>
          <text>std_env_args_CALL</text>
        </name>
      </transition>
      <transition id="std_iter_ExactSizeIterator_len_CALL">
        <name>
          <text>std_iter_ExactSizeIterator_len_CALL</text>
        </name>
      </transition>
      <transition id="std_iter_ExactSizeIterator_len_CALL_UNWIND">
        <name>
          <text>std_iter_ExactSizeIterator_len_CALL_UNWIND</text>
        </name>
      </transition>
      <arc source="PROGRAM_START" target="std_env_args_CALL" id="(PROGRAM_START, std_env_args_CALL)">
        <name>
          <text>(PROGRAM_START, std_env_args_CALL)</text>
        </name>
        <inscription>
          <text>1</text>
        </inscription>
      </arc>
      <arc source="main_BB1" target="main_BB1_STMT0" id="(main_BB1, main_BB1_STMT0)">
        <name>
          <text>(main_BB1, main_BB1_STMT0)</text>
        </name>
        <inscription>
          <text>1</text>
        </inscription>
      </arc>
      <arc source="main_BB1_END_PLACE" target="std_iter_ExactSizeIterator_len_CALL" id="(main_BB1_END_PLACE, std_iter_ExactSizeIterator_len_CALL)">
        <name>
          <text>(main_BB1_END_PLACE, std_iter_ExactSizeIterator_len_CALL)</text>
        </name>
        <inscription>
          <text>1</text>
        </inscription>
      </arc>
      <arc source="main_BB1_END_PLACE" target="std_iter_ExactSizeIterator_len_CALL_UNWIND" id="(main_BB1_END_PLACE, std_iter_ExactSizeIterator_len_CALL_UNWIND)">
        <name>
          <text>(main_BB1_END_PLACE, std_iter_ExactSizeIterator_len_CALL_UNWIND)</text>
        </name>
        <inscription>
          <text>1</text>
        </inscription>
      </arc>
      <arc source="main_BB2" target="main_DROP_2" id="(main_BB2, main_DROP_2)">
        <name>
          <text>(main_BB2, main_DROP_2)</text>
        </name>
        <inscription>
          <text>1</text>
        </inscription>
      </arc>
      <arc source="main_BB3" target="main_RETURN" id="(main_BB3, main_RETURN)">
        <name>
          <text>(main_BB3, main_RETURN)</text>
        </name>
        <inscription>
          <text>1</text>
        </inscription>
      </arc>
      <arc source="main_BB4" target="main_DROP_4" id="(main_BB4, main_DROP_4)">
        <name>
          <text>(main_BB4, main_DROP_4)</text>
        </name>
        <inscription>
          <text>1</text>
        </inscription>
      </arc>
      <arc source="main_BB5" target="main_UNWIND_5" id="(main_BB5, main_UNWIND_5)">
        <name>
          <text>(main_BB5, main_UNWIND_5)</text>
        </name>
        <inscription>
          <text>1</text>
        </inscription>
      </arc>
      <arc source="main_BB1_STMT0" target="main_BB1_END_PLACE" id="(main_BB1_STMT0, main_BB1_END_PLACE)">
        <name>
          <text>(main_BB1_STMT0, main_BB1_END_PLACE)</text>
        </name>
        <inscription>
          <text>1</text>
        </inscription>
      </arc>
      <arc source="main_DROP_2" target="main_BB3" id="(main_DROP_2, main_BB3)">
        <name>
          <text>(main_DROP_2, main_BB3)</text>
        </name>
        <inscription>
          <text>1</text>
        </inscription>
      </arc>
      <arc source="main_DROP_4" target="main_BB5" id="(main_DROP_4, main_BB5)">
        <name>
          <text>(main_DROP_4, main_BB5)</text>
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
      <arc source="main_UNWIND_5" target="PROGRAM_PANIC" id="(main_UNWIND_5, PROGRAM_PANIC)">
        <name>
          <text>(main_UNWIND_5, PROGRAM_PANIC)</text>
        </name>
        <inscription>
          <text>1</text>
        </inscription>
      </arc>
      <arc source="std_env_args_CALL" target="main_BB1" id="(std_env_args_CALL, main_BB1)">
        <name>
          <text>(std_env_args_CALL, main_BB1)</text>
        </name>
        <inscription>
          <text>1</text>
        </inscription>
      </arc>
      <arc source="std_iter_ExactSizeIterator_len_CALL" target="main_BB2" id="(std_iter_ExactSizeIterator_len_CALL, main_BB2)">
        <name>
          <text>(std_iter_ExactSizeIterator_len_CALL, main_BB2)</text>
        </name>
        <inscription>
          <text>1</text>
        </inscription>
      </arc>
      <arc source="std_iter_ExactSizeIterator_len_CALL_UNWIND" target="main_BB4" id="(std_iter_ExactSizeIterator_len_CALL_UNWIND, main_BB4)">
        <name>
          <text>(std_iter_ExactSizeIterator_len_CALL_UNWIND, main_BB4)</text>
        </name>
        <inscription>
          <text>1</text>
        </inscription>
      </arc>
    </page>
  </net>
</pnml>"#;

#[test]
fn minimal_match_statement_generates_correct_dot_output_file() {
    assert_output_file(
        "./tests/sample_programs/minimal_match_statement.rs",
        "dot",
        "./net.dot",
        MINIMAL_MATCH_STATEMENT_DOT_OUTPUT,
    );
}

#[test]
fn minimal_match_statement_generates_correct_lola_output_file() {
    assert_output_file(
        "./tests/sample_programs/minimal_match_statement.rs",
        "lola",
        "./net.lola",
        MINIMAL_MATCH_STATEMENT_LOLA_OUTPUT,
    );
}

#[test]
fn minimal_match_statement_generates_correct_pnml_output_file() {
    assert_output_file(
        "./tests/sample_programs/minimal_match_statement.rs",
        "pnml",
        "./net.pnml",
        MINIMAL_MATCH_STATEMENT_PNML_OUTPUT,
    );
}
