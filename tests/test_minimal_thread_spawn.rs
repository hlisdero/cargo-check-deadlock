mod common;
use common::assert_output_file;

const MINIMAL_THREAD_SPAWN_DOT_OUTPUT: &str = r#"digraph petrinet {
    PROGRAM_END [shape="circle" xlabel="PROGRAM_END" label=""];
    PROGRAM_PANIC [shape="circle" xlabel="PROGRAM_PANIC" label=""];
    PROGRAM_START [shape="circle" xlabel="PROGRAM_START" label="•"];
    THREAD_END_0 [shape="circle" xlabel="THREAD_END_0" label=""];
    THREAD_START_0 [shape="circle" xlabel="THREAD_START_0" label=""];
    main_BB1 [shape="circle" xlabel="main_BB1" label=""];
    main_BB1_END_PLACE [shape="circle" xlabel="main_BB1_END_PLACE" label=""];
    main_BB2 [shape="circle" xlabel="main_BB2" label=""];
    main_BB3 [shape="circle" xlabel="main_BB3" label=""];
    main_BB1_STMT0 [shape="box" xlabel="" label="main_BB1_STMT0"];
    main_DROP_2 [shape="box" xlabel="" label="main_DROP_2"];
    main_RETURN [shape="box" xlabel="" label="main_RETURN"];
    main__closure_0__RETURN [shape="box" xlabel="" label="main__closure_0__RETURN"];
    std_thread_JoinHandle_T_join_0 [shape="box" xlabel="" label="std_thread_JoinHandle_T_join_0"];
    std_thread_spawn_0 [shape="box" xlabel="" label="std_thread_spawn_0"];
    PROGRAM_START -> std_thread_spawn_0;
    THREAD_END_0 -> std_thread_JoinHandle_T_join_0;
    THREAD_START_0 -> main__closure_0__RETURN;
    main_BB1 -> main_BB1_STMT0;
    main_BB1_END_PLACE -> std_thread_JoinHandle_T_join_0;
    main_BB2 -> main_DROP_2;
    main_BB3 -> main_RETURN;
    main_BB1_STMT0 -> main_BB1_END_PLACE;
    main_DROP_2 -> main_BB3;
    main_RETURN -> PROGRAM_END;
    main__closure_0__RETURN -> THREAD_END_0;
    std_thread_JoinHandle_T_join_0 -> main_BB2;
    std_thread_spawn_0 -> THREAD_START_0;
    std_thread_spawn_0 -> main_BB1;
}
"#;

const MINIMAL_THREAD_SPAWN_LOLA_OUTPUT: &str = r#"PLACE
    PROGRAM_END,
    PROGRAM_PANIC,
    PROGRAM_START,
    THREAD_END_0,
    THREAD_START_0,
    main_BB1,
    main_BB1_END_PLACE,
    main_BB2,
    main_BB3;

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
TRANSITION main_RETURN
  CONSUME
    main_BB3 : 1;
  PRODUCE
    PROGRAM_END : 1;
TRANSITION main__closure_0__RETURN
  CONSUME
    THREAD_START_0 : 1;
  PRODUCE
    THREAD_END_0 : 1;
TRANSITION std_thread_JoinHandle_T_join_0
  CONSUME
    THREAD_END_0 : 1,
    main_BB1_END_PLACE : 1;
  PRODUCE
    main_BB2 : 1;
TRANSITION std_thread_spawn_0
  CONSUME
    PROGRAM_START : 1;
  PRODUCE
    THREAD_START_0 : 1,
    main_BB1 : 1;
"#;

const MINIMAL_THREAD_SPAWN_PNML_OUTPUT: &str = r#"<?xml version="1.0" encoding="utf-8"?>
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
      <place id="THREAD_END_0">
        <name>
          <text>THREAD_END_0</text>
        </name>
      </place>
      <place id="THREAD_START_0">
        <name>
          <text>THREAD_START_0</text>
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
      <place id="main_BB3">
        <name>
          <text>main_BB3</text>
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
      <transition id="main_RETURN">
        <name>
          <text>main_RETURN</text>
        </name>
      </transition>
      <transition id="main__closure_0__RETURN">
        <name>
          <text>main__closure_0__RETURN</text>
        </name>
      </transition>
      <transition id="std_thread_JoinHandle_T_join_0">
        <name>
          <text>std_thread_JoinHandle_T_join_0</text>
        </name>
      </transition>
      <transition id="std_thread_spawn_0">
        <name>
          <text>std_thread_spawn_0</text>
        </name>
      </transition>
      <arc source="PROGRAM_START" target="std_thread_spawn_0" id="(PROGRAM_START, std_thread_spawn_0)">
        <name>
          <text>(PROGRAM_START, std_thread_spawn_0)</text>
        </name>
        <inscription>
          <text>1</text>
        </inscription>
      </arc>
      <arc source="THREAD_END_0" target="std_thread_JoinHandle_T_join_0" id="(THREAD_END_0, std_thread_JoinHandle_T_join_0)">
        <name>
          <text>(THREAD_END_0, std_thread_JoinHandle_T_join_0)</text>
        </name>
        <inscription>
          <text>1</text>
        </inscription>
      </arc>
      <arc source="THREAD_START_0" target="main__closure_0__RETURN" id="(THREAD_START_0, main__closure_0__RETURN)">
        <name>
          <text>(THREAD_START_0, main__closure_0__RETURN)</text>
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
      <arc source="main_BB1_END_PLACE" target="std_thread_JoinHandle_T_join_0" id="(main_BB1_END_PLACE, std_thread_JoinHandle_T_join_0)">
        <name>
          <text>(main_BB1_END_PLACE, std_thread_JoinHandle_T_join_0)</text>
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
      <arc source="main_RETURN" target="PROGRAM_END" id="(main_RETURN, PROGRAM_END)">
        <name>
          <text>(main_RETURN, PROGRAM_END)</text>
        </name>
        <inscription>
          <text>1</text>
        </inscription>
      </arc>
      <arc source="main__closure_0__RETURN" target="THREAD_END_0" id="(main__closure_0__RETURN, THREAD_END_0)">
        <name>
          <text>(main__closure_0__RETURN, THREAD_END_0)</text>
        </name>
        <inscription>
          <text>1</text>
        </inscription>
      </arc>
      <arc source="std_thread_JoinHandle_T_join_0" target="main_BB2" id="(std_thread_JoinHandle_T_join_0, main_BB2)">
        <name>
          <text>(std_thread_JoinHandle_T_join_0, main_BB2)</text>
        </name>
        <inscription>
          <text>1</text>
        </inscription>
      </arc>
      <arc source="std_thread_spawn_0" target="THREAD_START_0" id="(std_thread_spawn_0, THREAD_START_0)">
        <name>
          <text>(std_thread_spawn_0, THREAD_START_0)</text>
        </name>
        <inscription>
          <text>1</text>
        </inscription>
      </arc>
      <arc source="std_thread_spawn_0" target="main_BB1" id="(std_thread_spawn_0, main_BB1)">
        <name>
          <text>(std_thread_spawn_0, main_BB1)</text>
        </name>
        <inscription>
          <text>1</text>
        </inscription>
      </arc>
    </page>
  </net>
</pnml>"#;

#[test]
fn minimal_thread_spawn_generates_correct_dot_output_file() {
    assert_output_file(
        "./tests/sample_programs/minimal_thread_spawn.rs",
        "dot",
        "./net.dot",
        MINIMAL_THREAD_SPAWN_DOT_OUTPUT,
    );
}

#[test]
fn minimal_thread_spawn_generates_correct_lola_output_file() {
    assert_output_file(
        "./tests/sample_programs/minimal_thread_spawn.rs",
        "lola",
        "./net.lola",
        MINIMAL_THREAD_SPAWN_LOLA_OUTPUT,
    );
}

#[test]
fn minimal_thread_spawn_generates_correct_pnml_output_file() {
    assert_output_file(
        "./tests/sample_programs/minimal_thread_spawn.rs",
        "pnml",
        "./net.pnml",
        MINIMAL_THREAD_SPAWN_PNML_OUTPUT,
    );
}
