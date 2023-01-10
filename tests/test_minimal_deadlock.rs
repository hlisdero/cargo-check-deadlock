mod common;
use common::assert_output_file;

const MINIMAL_DEADLOCK_DOT_OUTPUT: &str = r#"digraph petrinet {
    MUTEX_0 [shape="circle" xlabel="MUTEX_0" label="•"];
    PROGRAM_END [shape="circle" xlabel="PROGRAM_END" label=""];
    PROGRAM_PANIC [shape="circle" xlabel="PROGRAM_PANIC" label=""];
    PROGRAM_START [shape="circle" xlabel="PROGRAM_START" label="•"];
    main_BASIC_BLOCK_1 [shape="circle" xlabel="main_BASIC_BLOCK_1" label=""];
    main_BASIC_BLOCK_2 [shape="circle" xlabel="main_BASIC_BLOCK_2" label=""];
    main_BASIC_BLOCK_3 [shape="circle" xlabel="main_BASIC_BLOCK_3" label=""];
    main_BASIC_BLOCK_4 [shape="circle" xlabel="main_BASIC_BLOCK_4" label=""];
    main_BASIC_BLOCK_5 [shape="circle" xlabel="main_BASIC_BLOCK_5" label=""];
    main_BASIC_BLOCK_6 [shape="circle" xlabel="main_BASIC_BLOCK_6" label=""];
    main_BASIC_BLOCK_7 [shape="circle" xlabel="main_BASIC_BLOCK_7" label=""];
    main_BASIC_BLOCK_END_PLACE_1 [shape="circle" xlabel="main_BASIC_BLOCK_END_PLACE_1" label=""];
    main_BASIC_BLOCK_END_PLACE_2 [shape="circle" xlabel="main_BASIC_BLOCK_END_PLACE_2" label=""];
    main_BLOCK_1_STATEMENT_0 [shape="box" xlabel="main_BLOCK_1_STATEMENT_0" label=""];
    main_BLOCK_2_STATEMENT_0 [shape="box" xlabel="main_BLOCK_2_STATEMENT_0" label=""];
    main_DROP_3 [shape="box" xlabel="main_DROP_3" label=""];
    main_DROP_4 [shape="box" xlabel="main_DROP_4" label=""];
    main_DROP_6 [shape="box" xlabel="main_DROP_6" label=""];
    main_DROP_UNWIND_3 [shape="box" xlabel="main_DROP_UNWIND_3" label=""];
    main_RETURN [shape="box" xlabel="main_RETURN" label=""];
    main_UNWIND_7 [shape="box" xlabel="main_UNWIND_7" label=""];
    std_sync_Mutex_T_lock_0 [shape="box" xlabel="std_sync_Mutex_T_lock_0" label=""];
    std_sync_Mutex_T_lock_1 [shape="box" xlabel="std_sync_Mutex_T_lock_1" label=""];
    std_sync_Mutex_T_new_0 [shape="box" xlabel="std_sync_Mutex_T_new_0" label=""];
    MUTEX_0 -> std_sync_Mutex_T_lock_0;
    MUTEX_0 -> std_sync_Mutex_T_lock_1;
    PROGRAM_START -> std_sync_Mutex_T_new_0;
    main_BASIC_BLOCK_1 -> main_BLOCK_1_STATEMENT_0;
    main_BASIC_BLOCK_2 -> main_BLOCK_2_STATEMENT_0;
    main_BASIC_BLOCK_3 -> main_DROP_3;
    main_BASIC_BLOCK_3 -> main_DROP_UNWIND_3;
    main_BASIC_BLOCK_4 -> main_DROP_4;
    main_BASIC_BLOCK_5 -> main_RETURN;
    main_BASIC_BLOCK_6 -> main_DROP_6;
    main_BASIC_BLOCK_7 -> main_UNWIND_7;
    main_BASIC_BLOCK_END_PLACE_1 -> std_sync_Mutex_T_lock_0;
    main_BASIC_BLOCK_END_PLACE_2 -> std_sync_Mutex_T_lock_1;
    main_BLOCK_1_STATEMENT_0 -> main_BASIC_BLOCK_END_PLACE_1;
    main_BLOCK_2_STATEMENT_0 -> main_BASIC_BLOCK_END_PLACE_2;
    main_DROP_3 -> main_BASIC_BLOCK_4;
    main_DROP_4 -> main_BASIC_BLOCK_5;
    main_DROP_6 -> main_BASIC_BLOCK_7;
    main_DROP_UNWIND_3 -> main_BASIC_BLOCK_6;
    main_RETURN -> PROGRAM_END;
    main_UNWIND_7 -> PROGRAM_PANIC;
    std_sync_Mutex_T_lock_0 -> main_BASIC_BLOCK_2;
    std_sync_Mutex_T_lock_1 -> main_BASIC_BLOCK_3;
    std_sync_Mutex_T_lock_1 -> main_BASIC_BLOCK_6;
    std_sync_Mutex_T_new_0 -> main_BASIC_BLOCK_1;
}
"#;

const MINIMAL_DEADLOCK_LOLA_OUTPUT: &str = r#"PLACE
    MUTEX_0,
    PROGRAM_END,
    PROGRAM_PANIC,
    PROGRAM_START,
    main_BASIC_BLOCK_1,
    main_BASIC_BLOCK_2,
    main_BASIC_BLOCK_3,
    main_BASIC_BLOCK_4,
    main_BASIC_BLOCK_5,
    main_BASIC_BLOCK_6,
    main_BASIC_BLOCK_7,
    main_BASIC_BLOCK_END_PLACE_1,
    main_BASIC_BLOCK_END_PLACE_2;

MARKING
    MUTEX_0 : 1,
    PROGRAM_START : 1,
TRANSITION main_BLOCK_1_STATEMENT_0
  CONSUME
    main_BASIC_BLOCK_1 : 1;
  PRODUCE
    main_BASIC_BLOCK_END_PLACE_1 : 1;
TRANSITION main_BLOCK_2_STATEMENT_0
  CONSUME
    main_BASIC_BLOCK_2 : 1;
  PRODUCE
    main_BASIC_BLOCK_END_PLACE_2 : 1;
TRANSITION main_DROP_3
  CONSUME
    main_BASIC_BLOCK_3 : 1;
  PRODUCE
    main_BASIC_BLOCK_4 : 1;
TRANSITION main_DROP_4
  CONSUME
    main_BASIC_BLOCK_4 : 1;
  PRODUCE
    main_BASIC_BLOCK_5 : 1;
TRANSITION main_DROP_6
  CONSUME
    main_BASIC_BLOCK_6 : 1;
  PRODUCE
    main_BASIC_BLOCK_7 : 1;
TRANSITION main_DROP_UNWIND_3
  CONSUME
    main_BASIC_BLOCK_3 : 1;
  PRODUCE
    main_BASIC_BLOCK_6 : 1;
TRANSITION main_RETURN
  CONSUME
    main_BASIC_BLOCK_5 : 1;
  PRODUCE
    PROGRAM_END : 1;
TRANSITION main_UNWIND_7
  CONSUME
    main_BASIC_BLOCK_7 : 1;
  PRODUCE
    PROGRAM_PANIC : 1;
TRANSITION std_sync_Mutex_T_lock_0
  CONSUME
    MUTEX_0 : 1,
    main_BASIC_BLOCK_END_PLACE_1 : 1;
  PRODUCE
    main_BASIC_BLOCK_2 : 1;
TRANSITION std_sync_Mutex_T_lock_1
  CONSUME
    MUTEX_0 : 1,
    main_BASIC_BLOCK_END_PLACE_2 : 1;
  PRODUCE
    main_BASIC_BLOCK_3 : 1,
    main_BASIC_BLOCK_6 : 1;
TRANSITION std_sync_Mutex_T_new_0
  CONSUME
    PROGRAM_START : 1;
  PRODUCE
    main_BASIC_BLOCK_1 : 1;
"#;

const MINIMAL_DEADLOCK_PNML_OUTPUT: &str = r#"<?xml version="1.0" encoding="utf-8"?>
<pnml xmlns="http://www.pnml.org/version-2009/grammar/pnml">
  <net id="net0" type="http://www.pnml.org/version-2009/grammar/ptnet">
    <page id="page0">
      <place id="MUTEX_0">
        <name>
          <text>MUTEX_0</text>
        </name>
        <initialMarking>
          <text>1</text>
        </initialMarking>
      </place>
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
      <place id="main_BASIC_BLOCK_2">
        <name>
          <text>main_BASIC_BLOCK_2</text>
        </name>
      </place>
      <place id="main_BASIC_BLOCK_3">
        <name>
          <text>main_BASIC_BLOCK_3</text>
        </name>
      </place>
      <place id="main_BASIC_BLOCK_4">
        <name>
          <text>main_BASIC_BLOCK_4</text>
        </name>
      </place>
      <place id="main_BASIC_BLOCK_5">
        <name>
          <text>main_BASIC_BLOCK_5</text>
        </name>
      </place>
      <place id="main_BASIC_BLOCK_6">
        <name>
          <text>main_BASIC_BLOCK_6</text>
        </name>
      </place>
      <place id="main_BASIC_BLOCK_7">
        <name>
          <text>main_BASIC_BLOCK_7</text>
        </name>
      </place>
      <place id="main_BASIC_BLOCK_END_PLACE_1">
        <name>
          <text>main_BASIC_BLOCK_END_PLACE_1</text>
        </name>
      </place>
      <place id="main_BASIC_BLOCK_END_PLACE_2">
        <name>
          <text>main_BASIC_BLOCK_END_PLACE_2</text>
        </name>
      </place>
      <transition id="main_BLOCK_1_STATEMENT_0">
        <name>
          <text>main_BLOCK_1_STATEMENT_0</text>
        </name>
      </transition>
      <transition id="main_BLOCK_2_STATEMENT_0">
        <name>
          <text>main_BLOCK_2_STATEMENT_0</text>
        </name>
      </transition>
      <transition id="main_DROP_3">
        <name>
          <text>main_DROP_3</text>
        </name>
      </transition>
      <transition id="main_DROP_4">
        <name>
          <text>main_DROP_4</text>
        </name>
      </transition>
      <transition id="main_DROP_6">
        <name>
          <text>main_DROP_6</text>
        </name>
      </transition>
      <transition id="main_DROP_UNWIND_3">
        <name>
          <text>main_DROP_UNWIND_3</text>
        </name>
      </transition>
      <transition id="main_RETURN">
        <name>
          <text>main_RETURN</text>
        </name>
      </transition>
      <transition id="main_UNWIND_7">
        <name>
          <text>main_UNWIND_7</text>
        </name>
      </transition>
      <transition id="std_sync_Mutex_T_lock_0">
        <name>
          <text>std_sync_Mutex_T_lock_0</text>
        </name>
      </transition>
      <transition id="std_sync_Mutex_T_lock_1">
        <name>
          <text>std_sync_Mutex_T_lock_1</text>
        </name>
      </transition>
      <transition id="std_sync_Mutex_T_new_0">
        <name>
          <text>std_sync_Mutex_T_new_0</text>
        </name>
      </transition>
      <arc source="MUTEX_0" target="std_sync_Mutex_T_lock_0" id="(MUTEX_0, std_sync_Mutex_T_lock_0)">
        <name>
          <text>(MUTEX_0, std_sync_Mutex_T_lock_0)</text>
        </name>
        <inscription>
          <text>1</text>
        </inscription>
      </arc>
      <arc source="MUTEX_0" target="std_sync_Mutex_T_lock_1" id="(MUTEX_0, std_sync_Mutex_T_lock_1)">
        <name>
          <text>(MUTEX_0, std_sync_Mutex_T_lock_1)</text>
        </name>
        <inscription>
          <text>1</text>
        </inscription>
      </arc>
      <arc source="PROGRAM_START" target="std_sync_Mutex_T_new_0" id="(PROGRAM_START, std_sync_Mutex_T_new_0)">
        <name>
          <text>(PROGRAM_START, std_sync_Mutex_T_new_0)</text>
        </name>
        <inscription>
          <text>1</text>
        </inscription>
      </arc>
      <arc source="main_BASIC_BLOCK_1" target="main_BLOCK_1_STATEMENT_0" id="(main_BASIC_BLOCK_1, main_BLOCK_1_STATEMENT_0)">
        <name>
          <text>(main_BASIC_BLOCK_1, main_BLOCK_1_STATEMENT_0)</text>
        </name>
        <inscription>
          <text>1</text>
        </inscription>
      </arc>
      <arc source="main_BASIC_BLOCK_2" target="main_BLOCK_2_STATEMENT_0" id="(main_BASIC_BLOCK_2, main_BLOCK_2_STATEMENT_0)">
        <name>
          <text>(main_BASIC_BLOCK_2, main_BLOCK_2_STATEMENT_0)</text>
        </name>
        <inscription>
          <text>1</text>
        </inscription>
      </arc>
      <arc source="main_BASIC_BLOCK_3" target="main_DROP_3" id="(main_BASIC_BLOCK_3, main_DROP_3)">
        <name>
          <text>(main_BASIC_BLOCK_3, main_DROP_3)</text>
        </name>
        <inscription>
          <text>1</text>
        </inscription>
      </arc>
      <arc source="main_BASIC_BLOCK_3" target="main_DROP_UNWIND_3" id="(main_BASIC_BLOCK_3, main_DROP_UNWIND_3)">
        <name>
          <text>(main_BASIC_BLOCK_3, main_DROP_UNWIND_3)</text>
        </name>
        <inscription>
          <text>1</text>
        </inscription>
      </arc>
      <arc source="main_BASIC_BLOCK_4" target="main_DROP_4" id="(main_BASIC_BLOCK_4, main_DROP_4)">
        <name>
          <text>(main_BASIC_BLOCK_4, main_DROP_4)</text>
        </name>
        <inscription>
          <text>1</text>
        </inscription>
      </arc>
      <arc source="main_BASIC_BLOCK_5" target="main_RETURN" id="(main_BASIC_BLOCK_5, main_RETURN)">
        <name>
          <text>(main_BASIC_BLOCK_5, main_RETURN)</text>
        </name>
        <inscription>
          <text>1</text>
        </inscription>
      </arc>
      <arc source="main_BASIC_BLOCK_6" target="main_DROP_6" id="(main_BASIC_BLOCK_6, main_DROP_6)">
        <name>
          <text>(main_BASIC_BLOCK_6, main_DROP_6)</text>
        </name>
        <inscription>
          <text>1</text>
        </inscription>
      </arc>
      <arc source="main_BASIC_BLOCK_7" target="main_UNWIND_7" id="(main_BASIC_BLOCK_7, main_UNWIND_7)">
        <name>
          <text>(main_BASIC_BLOCK_7, main_UNWIND_7)</text>
        </name>
        <inscription>
          <text>1</text>
        </inscription>
      </arc>
      <arc source="main_BASIC_BLOCK_END_PLACE_1" target="std_sync_Mutex_T_lock_0" id="(main_BASIC_BLOCK_END_PLACE_1, std_sync_Mutex_T_lock_0)">
        <name>
          <text>(main_BASIC_BLOCK_END_PLACE_1, std_sync_Mutex_T_lock_0)</text>
        </name>
        <inscription>
          <text>1</text>
        </inscription>
      </arc>
      <arc source="main_BASIC_BLOCK_END_PLACE_2" target="std_sync_Mutex_T_lock_1" id="(main_BASIC_BLOCK_END_PLACE_2, std_sync_Mutex_T_lock_1)">
        <name>
          <text>(main_BASIC_BLOCK_END_PLACE_2, std_sync_Mutex_T_lock_1)</text>
        </name>
        <inscription>
          <text>1</text>
        </inscription>
      </arc>
      <arc source="main_BLOCK_1_STATEMENT_0" target="main_BASIC_BLOCK_END_PLACE_1" id="(main_BLOCK_1_STATEMENT_0, main_BASIC_BLOCK_END_PLACE_1)">
        <name>
          <text>(main_BLOCK_1_STATEMENT_0, main_BASIC_BLOCK_END_PLACE_1)</text>
        </name>
        <inscription>
          <text>1</text>
        </inscription>
      </arc>
      <arc source="main_BLOCK_2_STATEMENT_0" target="main_BASIC_BLOCK_END_PLACE_2" id="(main_BLOCK_2_STATEMENT_0, main_BASIC_BLOCK_END_PLACE_2)">
        <name>
          <text>(main_BLOCK_2_STATEMENT_0, main_BASIC_BLOCK_END_PLACE_2)</text>
        </name>
        <inscription>
          <text>1</text>
        </inscription>
      </arc>
      <arc source="main_DROP_3" target="main_BASIC_BLOCK_4" id="(main_DROP_3, main_BASIC_BLOCK_4)">
        <name>
          <text>(main_DROP_3, main_BASIC_BLOCK_4)</text>
        </name>
        <inscription>
          <text>1</text>
        </inscription>
      </arc>
      <arc source="main_DROP_4" target="main_BASIC_BLOCK_5" id="(main_DROP_4, main_BASIC_BLOCK_5)">
        <name>
          <text>(main_DROP_4, main_BASIC_BLOCK_5)</text>
        </name>
        <inscription>
          <text>1</text>
        </inscription>
      </arc>
      <arc source="main_DROP_6" target="main_BASIC_BLOCK_7" id="(main_DROP_6, main_BASIC_BLOCK_7)">
        <name>
          <text>(main_DROP_6, main_BASIC_BLOCK_7)</text>
        </name>
        <inscription>
          <text>1</text>
        </inscription>
      </arc>
      <arc source="main_DROP_UNWIND_3" target="main_BASIC_BLOCK_6" id="(main_DROP_UNWIND_3, main_BASIC_BLOCK_6)">
        <name>
          <text>(main_DROP_UNWIND_3, main_BASIC_BLOCK_6)</text>
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
      <arc source="main_UNWIND_7" target="PROGRAM_PANIC" id="(main_UNWIND_7, PROGRAM_PANIC)">
        <name>
          <text>(main_UNWIND_7, PROGRAM_PANIC)</text>
        </name>
        <inscription>
          <text>1</text>
        </inscription>
      </arc>
      <arc source="std_sync_Mutex_T_lock_0" target="main_BASIC_BLOCK_2" id="(std_sync_Mutex_T_lock_0, main_BASIC_BLOCK_2)">
        <name>
          <text>(std_sync_Mutex_T_lock_0, main_BASIC_BLOCK_2)</text>
        </name>
        <inscription>
          <text>1</text>
        </inscription>
      </arc>
      <arc source="std_sync_Mutex_T_lock_1" target="main_BASIC_BLOCK_3" id="(std_sync_Mutex_T_lock_1, main_BASIC_BLOCK_3)">
        <name>
          <text>(std_sync_Mutex_T_lock_1, main_BASIC_BLOCK_3)</text>
        </name>
        <inscription>
          <text>1</text>
        </inscription>
      </arc>
      <arc source="std_sync_Mutex_T_lock_1" target="main_BASIC_BLOCK_6" id="(std_sync_Mutex_T_lock_1, main_BASIC_BLOCK_6)">
        <name>
          <text>(std_sync_Mutex_T_lock_1, main_BASIC_BLOCK_6)</text>
        </name>
        <inscription>
          <text>1</text>
        </inscription>
      </arc>
      <arc source="std_sync_Mutex_T_new_0" target="main_BASIC_BLOCK_1" id="(std_sync_Mutex_T_new_0, main_BASIC_BLOCK_1)">
        <name>
          <text>(std_sync_Mutex_T_new_0, main_BASIC_BLOCK_1)</text>
        </name>
        <inscription>
          <text>1</text>
        </inscription>
      </arc>
    </page>
  </net>
</pnml>"#;

#[test]
fn minimal_deadlock_generates_correct_dot_output_file() {
    assert_output_file(
        "./tests/sample_programs/minimal_deadlock.rs",
        "dot",
        "./net.dot",
        MINIMAL_DEADLOCK_DOT_OUTPUT,
    );
}

#[test]
fn minimal_deadlock_generates_correct_lola_output_file() {
    assert_output_file(
        "./tests/sample_programs/minimal_deadlock.rs",
        "lola",
        "./net.lola",
        MINIMAL_DEADLOCK_LOLA_OUTPUT,
    );
}

#[test]
fn minimal_deadlock_generates_correct_pnml_output_file() {
    assert_output_file(
        "./tests/sample_programs/minimal_deadlock.rs",
        "pnml",
        "./net.pnml",
        MINIMAL_DEADLOCK_PNML_OUTPUT,
    );
}
