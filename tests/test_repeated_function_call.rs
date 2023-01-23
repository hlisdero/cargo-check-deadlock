mod common;
use common::assert_output_file;

const REPEATED_FUNCTION_CALL_DOT_OUTPUT: &str = r#"digraph petrinet {
    PROGRAM_END [shape="circle" xlabel="PROGRAM_END" label=""];
    PROGRAM_PANIC [shape="circle" xlabel="PROGRAM_PANIC" label=""];
    PROGRAM_START [shape="circle" xlabel="PROGRAM_START" label="•"];
    main_BB0_END_PLACE [shape="circle" xlabel="main_BB0_END_PLACE" label=""];
    main_BB0_STMT0_END [shape="circle" xlabel="main_BB0_STMT0_END" label=""];
    main_BB0_STMT1_END [shape="circle" xlabel="main_BB0_STMT1_END" label=""];
    main_BB1 [shape="circle" xlabel="main_BB1" label=""];
    main_BB1_END_PLACE [shape="circle" xlabel="main_BB1_END_PLACE" label=""];
    main_BB2 [shape="circle" xlabel="main_BB2" label=""];
    main_BB2_END_PLACE [shape="circle" xlabel="main_BB2_END_PLACE" label=""];
    main_BB2_STMT0_END [shape="circle" xlabel="main_BB2_STMT0_END" label=""];
    main_BB3 [shape="circle" xlabel="main_BB3" label=""];
    main_BB3_END_PLACE [shape="circle" xlabel="main_BB3_END_PLACE" label=""];
    main_BB4 [shape="circle" xlabel="main_BB4" label=""];
    main_BB4_END_PLACE [shape="circle" xlabel="main_BB4_END_PLACE" label=""];
    main_BB5 [shape="circle" xlabel="main_BB5" label=""];
    main_BB6 [shape="circle" xlabel="main_BB6" label=""];
    call_RETURN [shape="box" xlabel="" label="call_RETURN"];
    main_BB0_STMT0 [shape="box" xlabel="" label="main_BB0_STMT0"];
    main_BB0_STMT1 [shape="box" xlabel="" label="main_BB0_STMT1"];
    main_BB0_STMT2 [shape="box" xlabel="" label="main_BB0_STMT2"];
    main_BB1_STMT0 [shape="box" xlabel="" label="main_BB1_STMT0"];
    main_BB2_STMT0 [shape="box" xlabel="" label="main_BB2_STMT0"];
    main_BB2_STMT1 [shape="box" xlabel="" label="main_BB2_STMT1"];
    main_BB3_STMT0 [shape="box" xlabel="" label="main_BB3_STMT0"];
    main_BB4_STMT0 [shape="box" xlabel="" label="main_BB4_STMT0"];
    main_GOTO_1 [shape="box" xlabel="" label="main_GOTO_1"];
    main_RETURN [shape="box" xlabel="" label="main_RETURN"];
    main_SWITCH_INT_4 [shape="box" xlabel="" label="main_SWITCH_INT_4"];
    main_SWITCH_INT_5 [shape="box" xlabel="" label="main_SWITCH_INT_5"];
    main_SWITCH_INT_6 [shape="box" xlabel="" label="main_SWITCH_INT_6"];
    main_UNREACHABLE_5 [shape="box" xlabel="" label="main_UNREACHABLE_5"];
    std_iter_IntoIterator_into_iter_CALL [shape="box" xlabel="" label="std_iter_IntoIterator_into_iter_CALL"];
    std_iter_Iterator_next_CALL [shape="box" xlabel="" label="std_iter_Iterator_next_CALL"];
    PROGRAM_START -> main_BB0_STMT0;
    main_BB0_END_PLACE -> std_iter_IntoIterator_into_iter_CALL;
    main_BB0_STMT0_END -> main_BB0_STMT1;
    main_BB0_STMT1_END -> main_BB0_STMT2;
    main_BB1 -> main_BB1_STMT0;
    main_BB1_END_PLACE -> main_GOTO_1;
    main_BB2 -> main_BB2_STMT0;
    main_BB2_END_PLACE -> std_iter_Iterator_next_CALL;
    main_BB2_STMT0_END -> main_BB2_STMT1;
    main_BB3 -> main_BB3_STMT0;
    main_BB3_END_PLACE -> main_SWITCH_INT_4;
    main_BB3_END_PLACE -> main_SWITCH_INT_5;
    main_BB3_END_PLACE -> main_SWITCH_INT_6;
    main_BB4 -> main_BB4_STMT0;
    main_BB4_END_PLACE -> call_RETURN;
    main_BB5 -> main_UNREACHABLE_5;
    main_BB6 -> main_RETURN;
    call_RETURN -> main_BB2;
    main_BB0_STMT0 -> main_BB0_STMT0_END;
    main_BB0_STMT1 -> main_BB0_STMT1_END;
    main_BB0_STMT2 -> main_BB0_END_PLACE;
    main_BB1_STMT0 -> main_BB1_END_PLACE;
    main_BB2_STMT0 -> main_BB2_STMT0_END;
    main_BB2_STMT1 -> main_BB2_END_PLACE;
    main_BB3_STMT0 -> main_BB3_END_PLACE;
    main_BB4_STMT0 -> main_BB4_END_PLACE;
    main_GOTO_1 -> main_BB2;
    main_RETURN -> PROGRAM_END;
    main_SWITCH_INT_4 -> main_BB4;
    main_SWITCH_INT_5 -> main_BB5;
    main_SWITCH_INT_6 -> main_BB6;
    main_UNREACHABLE_5 -> PROGRAM_END;
    std_iter_IntoIterator_into_iter_CALL -> main_BB1;
    std_iter_Iterator_next_CALL -> main_BB3;
}
"#;

const REPEATED_FUNCTION_CALL_LOLA_OUTPUT: &str = r#"PLACE
    PROGRAM_END,
    PROGRAM_PANIC,
    PROGRAM_START,
    main_BB0_END_PLACE,
    main_BB0_STMT0_END,
    main_BB0_STMT1_END,
    main_BB1,
    main_BB1_END_PLACE,
    main_BB2,
    main_BB2_END_PLACE,
    main_BB2_STMT0_END,
    main_BB3,
    main_BB3_END_PLACE,
    main_BB4,
    main_BB4_END_PLACE,
    main_BB5,
    main_BB6;

MARKING
    PROGRAM_START : 1,
TRANSITION call_RETURN
  CONSUME
    main_BB4_END_PLACE : 1;
  PRODUCE
    main_BB2 : 1;
TRANSITION main_BB0_STMT0
  CONSUME
    PROGRAM_START : 1;
  PRODUCE
    main_BB0_STMT0_END : 1;
TRANSITION main_BB0_STMT1
  CONSUME
    main_BB0_STMT0_END : 1;
  PRODUCE
    main_BB0_STMT1_END : 1;
TRANSITION main_BB0_STMT2
  CONSUME
    main_BB0_STMT1_END : 1;
  PRODUCE
    main_BB0_END_PLACE : 1;
TRANSITION main_BB1_STMT0
  CONSUME
    main_BB1 : 1;
  PRODUCE
    main_BB1_END_PLACE : 1;
TRANSITION main_BB2_STMT0
  CONSUME
    main_BB2 : 1;
  PRODUCE
    main_BB2_STMT0_END : 1;
TRANSITION main_BB2_STMT1
  CONSUME
    main_BB2_STMT0_END : 1;
  PRODUCE
    main_BB2_END_PLACE : 1;
TRANSITION main_BB3_STMT0
  CONSUME
    main_BB3 : 1;
  PRODUCE
    main_BB3_END_PLACE : 1;
TRANSITION main_BB4_STMT0
  CONSUME
    main_BB4 : 1;
  PRODUCE
    main_BB4_END_PLACE : 1;
TRANSITION main_GOTO_1
  CONSUME
    main_BB1_END_PLACE : 1;
  PRODUCE
    main_BB2 : 1;
TRANSITION main_RETURN
  CONSUME
    main_BB6 : 1;
  PRODUCE
    PROGRAM_END : 1;
TRANSITION main_SWITCH_INT_4
  CONSUME
    main_BB3_END_PLACE : 1;
  PRODUCE
    main_BB4 : 1;
TRANSITION main_SWITCH_INT_5
  CONSUME
    main_BB3_END_PLACE : 1;
  PRODUCE
    main_BB5 : 1;
TRANSITION main_SWITCH_INT_6
  CONSUME
    main_BB3_END_PLACE : 1;
  PRODUCE
    main_BB6 : 1;
TRANSITION main_UNREACHABLE_5
  CONSUME
    main_BB5 : 1;
  PRODUCE
    PROGRAM_END : 1;
TRANSITION std_iter_IntoIterator_into_iter_CALL
  CONSUME
    main_BB0_END_PLACE : 1;
  PRODUCE
    main_BB1 : 1;
TRANSITION std_iter_Iterator_next_CALL
  CONSUME
    main_BB2_END_PLACE : 1;
  PRODUCE
    main_BB3 : 1;
"#;

const REPEATED_FUNCTION_CALL_PNML_OUTPUT: &str = r#"<?xml version="1.0" encoding="utf-8"?>
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
      <place id="main_BB0_END_PLACE">
        <name>
          <text>main_BB0_END_PLACE</text>
        </name>
      </place>
      <place id="main_BB0_STMT0_END">
        <name>
          <text>main_BB0_STMT0_END</text>
        </name>
      </place>
      <place id="main_BB0_STMT1_END">
        <name>
          <text>main_BB0_STMT1_END</text>
        </name>
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
      <place id="main_BB2_END_PLACE">
        <name>
          <text>main_BB2_END_PLACE</text>
        </name>
      </place>
      <place id="main_BB2_STMT0_END">
        <name>
          <text>main_BB2_STMT0_END</text>
        </name>
      </place>
      <place id="main_BB3">
        <name>
          <text>main_BB3</text>
        </name>
      </place>
      <place id="main_BB3_END_PLACE">
        <name>
          <text>main_BB3_END_PLACE</text>
        </name>
      </place>
      <place id="main_BB4">
        <name>
          <text>main_BB4</text>
        </name>
      </place>
      <place id="main_BB4_END_PLACE">
        <name>
          <text>main_BB4_END_PLACE</text>
        </name>
      </place>
      <place id="main_BB5">
        <name>
          <text>main_BB5</text>
        </name>
      </place>
      <place id="main_BB6">
        <name>
          <text>main_BB6</text>
        </name>
      </place>
      <transition id="call_RETURN">
        <name>
          <text>call_RETURN</text>
        </name>
      </transition>
      <transition id="main_BB0_STMT0">
        <name>
          <text>main_BB0_STMT0</text>
        </name>
      </transition>
      <transition id="main_BB0_STMT1">
        <name>
          <text>main_BB0_STMT1</text>
        </name>
      </transition>
      <transition id="main_BB0_STMT2">
        <name>
          <text>main_BB0_STMT2</text>
        </name>
      </transition>
      <transition id="main_BB1_STMT0">
        <name>
          <text>main_BB1_STMT0</text>
        </name>
      </transition>
      <transition id="main_BB2_STMT0">
        <name>
          <text>main_BB2_STMT0</text>
        </name>
      </transition>
      <transition id="main_BB2_STMT1">
        <name>
          <text>main_BB2_STMT1</text>
        </name>
      </transition>
      <transition id="main_BB3_STMT0">
        <name>
          <text>main_BB3_STMT0</text>
        </name>
      </transition>
      <transition id="main_BB4_STMT0">
        <name>
          <text>main_BB4_STMT0</text>
        </name>
      </transition>
      <transition id="main_GOTO_1">
        <name>
          <text>main_GOTO_1</text>
        </name>
      </transition>
      <transition id="main_RETURN">
        <name>
          <text>main_RETURN</text>
        </name>
      </transition>
      <transition id="main_SWITCH_INT_4">
        <name>
          <text>main_SWITCH_INT_4</text>
        </name>
      </transition>
      <transition id="main_SWITCH_INT_5">
        <name>
          <text>main_SWITCH_INT_5</text>
        </name>
      </transition>
      <transition id="main_SWITCH_INT_6">
        <name>
          <text>main_SWITCH_INT_6</text>
        </name>
      </transition>
      <transition id="main_UNREACHABLE_5">
        <name>
          <text>main_UNREACHABLE_5</text>
        </name>
      </transition>
      <transition id="std_iter_IntoIterator_into_iter_CALL">
        <name>
          <text>std_iter_IntoIterator_into_iter_CALL</text>
        </name>
      </transition>
      <transition id="std_iter_Iterator_next_CALL">
        <name>
          <text>std_iter_Iterator_next_CALL</text>
        </name>
      </transition>
      <arc source="PROGRAM_START" target="main_BB0_STMT0" id="(PROGRAM_START, main_BB0_STMT0)">
        <name>
          <text>(PROGRAM_START, main_BB0_STMT0)</text>
        </name>
        <inscription>
          <text>1</text>
        </inscription>
      </arc>
      <arc source="main_BB0_END_PLACE" target="std_iter_IntoIterator_into_iter_CALL" id="(main_BB0_END_PLACE, std_iter_IntoIterator_into_iter_CALL)">
        <name>
          <text>(main_BB0_END_PLACE, std_iter_IntoIterator_into_iter_CALL)</text>
        </name>
        <inscription>
          <text>1</text>
        </inscription>
      </arc>
      <arc source="main_BB0_STMT0_END" target="main_BB0_STMT1" id="(main_BB0_STMT0_END, main_BB0_STMT1)">
        <name>
          <text>(main_BB0_STMT0_END, main_BB0_STMT1)</text>
        </name>
        <inscription>
          <text>1</text>
        </inscription>
      </arc>
      <arc source="main_BB0_STMT1_END" target="main_BB0_STMT2" id="(main_BB0_STMT1_END, main_BB0_STMT2)">
        <name>
          <text>(main_BB0_STMT1_END, main_BB0_STMT2)</text>
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
      <arc source="main_BB1_END_PLACE" target="main_GOTO_1" id="(main_BB1_END_PLACE, main_GOTO_1)">
        <name>
          <text>(main_BB1_END_PLACE, main_GOTO_1)</text>
        </name>
        <inscription>
          <text>1</text>
        </inscription>
      </arc>
      <arc source="main_BB2" target="main_BB2_STMT0" id="(main_BB2, main_BB2_STMT0)">
        <name>
          <text>(main_BB2, main_BB2_STMT0)</text>
        </name>
        <inscription>
          <text>1</text>
        </inscription>
      </arc>
      <arc source="main_BB2_END_PLACE" target="std_iter_Iterator_next_CALL" id="(main_BB2_END_PLACE, std_iter_Iterator_next_CALL)">
        <name>
          <text>(main_BB2_END_PLACE, std_iter_Iterator_next_CALL)</text>
        </name>
        <inscription>
          <text>1</text>
        </inscription>
      </arc>
      <arc source="main_BB2_STMT0_END" target="main_BB2_STMT1" id="(main_BB2_STMT0_END, main_BB2_STMT1)">
        <name>
          <text>(main_BB2_STMT0_END, main_BB2_STMT1)</text>
        </name>
        <inscription>
          <text>1</text>
        </inscription>
      </arc>
      <arc source="main_BB3" target="main_BB3_STMT0" id="(main_BB3, main_BB3_STMT0)">
        <name>
          <text>(main_BB3, main_BB3_STMT0)</text>
        </name>
        <inscription>
          <text>1</text>
        </inscription>
      </arc>
      <arc source="main_BB3_END_PLACE" target="main_SWITCH_INT_4" id="(main_BB3_END_PLACE, main_SWITCH_INT_4)">
        <name>
          <text>(main_BB3_END_PLACE, main_SWITCH_INT_4)</text>
        </name>
        <inscription>
          <text>1</text>
        </inscription>
      </arc>
      <arc source="main_BB3_END_PLACE" target="main_SWITCH_INT_5" id="(main_BB3_END_PLACE, main_SWITCH_INT_5)">
        <name>
          <text>(main_BB3_END_PLACE, main_SWITCH_INT_5)</text>
        </name>
        <inscription>
          <text>1</text>
        </inscription>
      </arc>
      <arc source="main_BB3_END_PLACE" target="main_SWITCH_INT_6" id="(main_BB3_END_PLACE, main_SWITCH_INT_6)">
        <name>
          <text>(main_BB3_END_PLACE, main_SWITCH_INT_6)</text>
        </name>
        <inscription>
          <text>1</text>
        </inscription>
      </arc>
      <arc source="main_BB4" target="main_BB4_STMT0" id="(main_BB4, main_BB4_STMT0)">
        <name>
          <text>(main_BB4, main_BB4_STMT0)</text>
        </name>
        <inscription>
          <text>1</text>
        </inscription>
      </arc>
      <arc source="main_BB4_END_PLACE" target="call_RETURN" id="(main_BB4_END_PLACE, call_RETURN)">
        <name>
          <text>(main_BB4_END_PLACE, call_RETURN)</text>
        </name>
        <inscription>
          <text>1</text>
        </inscription>
      </arc>
      <arc source="main_BB5" target="main_UNREACHABLE_5" id="(main_BB5, main_UNREACHABLE_5)">
        <name>
          <text>(main_BB5, main_UNREACHABLE_5)</text>
        </name>
        <inscription>
          <text>1</text>
        </inscription>
      </arc>
      <arc source="main_BB6" target="main_RETURN" id="(main_BB6, main_RETURN)">
        <name>
          <text>(main_BB6, main_RETURN)</text>
        </name>
        <inscription>
          <text>1</text>
        </inscription>
      </arc>
      <arc source="call_RETURN" target="main_BB2" id="(call_RETURN, main_BB2)">
        <name>
          <text>(call_RETURN, main_BB2)</text>
        </name>
        <inscription>
          <text>1</text>
        </inscription>
      </arc>
      <arc source="main_BB0_STMT0" target="main_BB0_STMT0_END" id="(main_BB0_STMT0, main_BB0_STMT0_END)">
        <name>
          <text>(main_BB0_STMT0, main_BB0_STMT0_END)</text>
        </name>
        <inscription>
          <text>1</text>
        </inscription>
      </arc>
      <arc source="main_BB0_STMT1" target="main_BB0_STMT1_END" id="(main_BB0_STMT1, main_BB0_STMT1_END)">
        <name>
          <text>(main_BB0_STMT1, main_BB0_STMT1_END)</text>
        </name>
        <inscription>
          <text>1</text>
        </inscription>
      </arc>
      <arc source="main_BB0_STMT2" target="main_BB0_END_PLACE" id="(main_BB0_STMT2, main_BB0_END_PLACE)">
        <name>
          <text>(main_BB0_STMT2, main_BB0_END_PLACE)</text>
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
      <arc source="main_BB2_STMT0" target="main_BB2_STMT0_END" id="(main_BB2_STMT0, main_BB2_STMT0_END)">
        <name>
          <text>(main_BB2_STMT0, main_BB2_STMT0_END)</text>
        </name>
        <inscription>
          <text>1</text>
        </inscription>
      </arc>
      <arc source="main_BB2_STMT1" target="main_BB2_END_PLACE" id="(main_BB2_STMT1, main_BB2_END_PLACE)">
        <name>
          <text>(main_BB2_STMT1, main_BB2_END_PLACE)</text>
        </name>
        <inscription>
          <text>1</text>
        </inscription>
      </arc>
      <arc source="main_BB3_STMT0" target="main_BB3_END_PLACE" id="(main_BB3_STMT0, main_BB3_END_PLACE)">
        <name>
          <text>(main_BB3_STMT0, main_BB3_END_PLACE)</text>
        </name>
        <inscription>
          <text>1</text>
        </inscription>
      </arc>
      <arc source="main_BB4_STMT0" target="main_BB4_END_PLACE" id="(main_BB4_STMT0, main_BB4_END_PLACE)">
        <name>
          <text>(main_BB4_STMT0, main_BB4_END_PLACE)</text>
        </name>
        <inscription>
          <text>1</text>
        </inscription>
      </arc>
      <arc source="main_GOTO_1" target="main_BB2" id="(main_GOTO_1, main_BB2)">
        <name>
          <text>(main_GOTO_1, main_BB2)</text>
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
      <arc source="main_SWITCH_INT_4" target="main_BB4" id="(main_SWITCH_INT_4, main_BB4)">
        <name>
          <text>(main_SWITCH_INT_4, main_BB4)</text>
        </name>
        <inscription>
          <text>1</text>
        </inscription>
      </arc>
      <arc source="main_SWITCH_INT_5" target="main_BB5" id="(main_SWITCH_INT_5, main_BB5)">
        <name>
          <text>(main_SWITCH_INT_5, main_BB5)</text>
        </name>
        <inscription>
          <text>1</text>
        </inscription>
      </arc>
      <arc source="main_SWITCH_INT_6" target="main_BB6" id="(main_SWITCH_INT_6, main_BB6)">
        <name>
          <text>(main_SWITCH_INT_6, main_BB6)</text>
        </name>
        <inscription>
          <text>1</text>
        </inscription>
      </arc>
      <arc source="main_UNREACHABLE_5" target="PROGRAM_END" id="(main_UNREACHABLE_5, PROGRAM_END)">
        <name>
          <text>(main_UNREACHABLE_5, PROGRAM_END)</text>
        </name>
        <inscription>
          <text>1</text>
        </inscription>
      </arc>
      <arc source="std_iter_IntoIterator_into_iter_CALL" target="main_BB1" id="(std_iter_IntoIterator_into_iter_CALL, main_BB1)">
        <name>
          <text>(std_iter_IntoIterator_into_iter_CALL, main_BB1)</text>
        </name>
        <inscription>
          <text>1</text>
        </inscription>
      </arc>
      <arc source="std_iter_Iterator_next_CALL" target="main_BB3" id="(std_iter_Iterator_next_CALL, main_BB3)">
        <name>
          <text>(std_iter_Iterator_next_CALL, main_BB3)</text>
        </name>
        <inscription>
          <text>1</text>
        </inscription>
      </arc>
    </page>
  </net>
</pnml>"#;

#[test]
fn repeated_function_call_generates_correct_dot_output_file() {
    assert_output_file(
        "./tests/sample_programs/repeated_function_call.rs",
        "dot",
        "./net.dot",
        REPEATED_FUNCTION_CALL_DOT_OUTPUT,
    );
}

#[test]
fn repeated_function_call_generates_correct_lola_output_file() {
    assert_output_file(
        "./tests/sample_programs/repeated_function_call.rs",
        "lola",
        "./net.lola",
        REPEATED_FUNCTION_CALL_LOLA_OUTPUT,
    );
}

#[test]
fn repeated_function_call_generates_correct_pnml_output_file() {
    assert_output_file(
        "./tests/sample_programs/repeated_function_call.rs",
        "pnml",
        "./net.pnml",
        REPEATED_FUNCTION_CALL_PNML_OUTPUT,
    );
}
