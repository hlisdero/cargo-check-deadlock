mod common;
use common::assert_output_file;

const MINIMAL_SHARED_MUTEX_DOT_OUTPUT: &str = r#"digraph petrinet {
    MUTEX_0 [shape="circle" xlabel="MUTEX_0" label="•"];
    PROGRAM_END [shape="circle" xlabel="PROGRAM_END" label=""];
    PROGRAM_PANIC [shape="circle" xlabel="PROGRAM_PANIC" label=""];
    PROGRAM_START [shape="circle" xlabel="PROGRAM_START" label="•"];
    THREAD_END_0 [shape="circle" xlabel="THREAD_END_0" label=""];
    THREAD_START_0 [shape="circle" xlabel="THREAD_START_0" label=""];
    main_BB0_END_PLACE [shape="circle" xlabel="main_BB0_END_PLACE" label=""];
    main_BB1 [shape="circle" xlabel="main_BB1" label=""];
    main_BB10 [shape="circle" xlabel="main_BB10" label=""];
    main_BB11 [shape="circle" xlabel="main_BB11" label=""];
    main_BB12 [shape="circle" xlabel="main_BB12" label=""];
    main_BB13 [shape="circle" xlabel="main_BB13" label=""];
    main_BB14 [shape="circle" xlabel="main_BB14" label=""];
    main_BB15 [shape="circle" xlabel="main_BB15" label=""];
    main_BB2 [shape="circle" xlabel="main_BB2" label=""];
    main_BB2_END_PLACE [shape="circle" xlabel="main_BB2_END_PLACE" label=""];
    main_BB3 [shape="circle" xlabel="main_BB3" label=""];
    main_BB3_END_PLACE [shape="circle" xlabel="main_BB3_END_PLACE" label=""];
    main_BB3_STMT0_END [shape="circle" xlabel="main_BB3_STMT0_END" label=""];
    main_BB4 [shape="circle" xlabel="main_BB4" label=""];
    main_BB4_END_PLACE [shape="circle" xlabel="main_BB4_END_PLACE" label=""];
    main_BB4_STMT0_END [shape="circle" xlabel="main_BB4_STMT0_END" label=""];
    main_BB5 [shape="circle" xlabel="main_BB5" label=""];
    main_BB5_END_PLACE [shape="circle" xlabel="main_BB5_END_PLACE" label=""];
    main_BB6 [shape="circle" xlabel="main_BB6" label=""];
    main_BB6_END_PLACE [shape="circle" xlabel="main_BB6_END_PLACE" label=""];
    main_BB6_STMT0_END [shape="circle" xlabel="main_BB6_STMT0_END" label=""];
    main_BB7 [shape="circle" xlabel="main_BB7" label=""];
    main_BB8 [shape="circle" xlabel="main_BB8" label=""];
    main_BB9 [shape="circle" xlabel="main_BB9" label=""];
    main_BB9_END_PLACE [shape="circle" xlabel="main_BB9_END_PLACE" label=""];
    main__closure_0__BB0_END_PLACE [shape="circle" xlabel="main__closure_0__BB0_END_PLACE" label=""];
    main__closure_0__BB1 [shape="circle" xlabel="main__closure_0__BB1" label=""];
    main__closure_0__BB1_END_PLACE [shape="circle" xlabel="main__closure_0__BB1_END_PLACE" label=""];
    main__closure_0__BB2 [shape="circle" xlabel="main__closure_0__BB2" label=""];
    main__closure_0__BB3 [shape="circle" xlabel="main__closure_0__BB3" label=""];
    main__closure_0__BB4 [shape="circle" xlabel="main__closure_0__BB4" label=""];
    main__closure_0__BB5 [shape="circle" xlabel="main__closure_0__BB5" label=""];
    main__closure_0__BB6 [shape="circle" xlabel="main__closure_0__BB6" label=""];
    main_BB0_STMT0 [shape="box" xlabel="" label="main_BB0_STMT0"];
    main_BB2_STMT0 [shape="box" xlabel="" label="main_BB2_STMT0"];
    main_BB3_STMT0 [shape="box" xlabel="" label="main_BB3_STMT0"];
    main_BB3_STMT1 [shape="box" xlabel="" label="main_BB3_STMT1"];
    main_BB4_STMT0 [shape="box" xlabel="" label="main_BB4_STMT0"];
    main_BB4_STMT1 [shape="box" xlabel="" label="main_BB4_STMT1"];
    main_BB5_STMT0 [shape="box" xlabel="" label="main_BB5_STMT0"];
    main_BB6_STMT0 [shape="box" xlabel="" label="main_BB6_STMT0"];
    main_BB6_STMT1 [shape="box" xlabel="" label="main_BB6_STMT1"];
    main_BB9_STMT0 [shape="box" xlabel="" label="main_BB9_STMT0"];
    main_DROP_11 [shape="box" xlabel="" label="main_DROP_11"];
    main_DROP_12 [shape="box" xlabel="" label="main_DROP_12"];
    main_DROP_14 [shape="box" xlabel="" label="main_DROP_14"];
    main_DROP_7 [shape="box" xlabel="" label="main_DROP_7"];
    main_DROP_8 [shape="box" xlabel="" label="main_DROP_8"];
    main_DROP_9 [shape="box" xlabel="" label="main_DROP_9"];
    main_DROP_UNWIND_7 [shape="box" xlabel="" label="main_DROP_UNWIND_7"];
    main_DROP_UNWIND_8 [shape="box" xlabel="" label="main_DROP_UNWIND_8"];
    main_RETURN [shape="box" xlabel="" label="main_RETURN"];
    main_SWITCH_INT_12 [shape="box" xlabel="" label="main_SWITCH_INT_12"];
    main_SWITCH_INT_14 [shape="box" xlabel="" label="main_SWITCH_INT_14"];
    main_UNWIND_13 [shape="box" xlabel="" label="main_UNWIND_13"];
    main__closure_0__BB0_STMT0 [shape="box" xlabel="" label="main__closure_0__BB0_STMT0"];
    main__closure_0__BB1_STMT0 [shape="box" xlabel="" label="main__closure_0__BB1_STMT0"];
    main__closure_0__DROP_2 [shape="box" xlabel="" label="main__closure_0__DROP_2"];
    main__closure_0__DROP_3 [shape="box" xlabel="" label="main__closure_0__DROP_3"];
    main__closure_0__DROP_5 [shape="box" xlabel="" label="main__closure_0__DROP_5"];
    main__closure_0__DROP_UNWIND_2 [shape="box" xlabel="" label="main__closure_0__DROP_UNWIND_2"];
    main__closure_0__RETURN [shape="box" xlabel="" label="main__closure_0__RETURN"];
    main__closure_0__UNWIND_6 [shape="box" xlabel="" label="main__closure_0__UNWIND_6"];
    std_clone_Clone_clone_0 [shape="box" xlabel="" label="std_clone_Clone_clone_0"];
    std_ops_Deref_deref_0 [shape="box" xlabel="" label="std_ops_Deref_deref_0"];
    std_ops_Deref_deref_1 [shape="box" xlabel="" label="std_ops_Deref_deref_1"];
    std_sync_Arc_T_new_0 [shape="box" xlabel="" label="std_sync_Arc_T_new_0"];
    std_sync_Mutex_T_lock_0 [shape="box" xlabel="" label="std_sync_Mutex_T_lock_0"];
    std_sync_Mutex_T_lock_1 [shape="box" xlabel="" label="std_sync_Mutex_T_lock_1"];
    std_sync_Mutex_T_new_0 [shape="box" xlabel="" label="std_sync_Mutex_T_new_0"];
    std_thread_JoinHandle_T_join_0 [shape="box" xlabel="" label="std_thread_JoinHandle_T_join_0"];
    std_thread_spawn_0 [shape="box" xlabel="" label="std_thread_spawn_0"];
    MUTEX_0 -> std_sync_Mutex_T_lock_0;
    MUTEX_0 -> std_sync_Mutex_T_lock_1;
    PROGRAM_START -> main_BB0_STMT0;
    THREAD_END_0 -> std_thread_JoinHandle_T_join_0;
    THREAD_START_0 -> main__closure_0__BB0_STMT0;
    main_BB0_END_PLACE -> std_sync_Mutex_T_new_0;
    main_BB1 -> std_sync_Arc_T_new_0;
    main_BB10 -> main_RETURN;
    main_BB11 -> main_DROP_11;
    main_BB12 -> main_DROP_12;
    main_BB13 -> main_UNWIND_13;
    main_BB14 -> main_DROP_14;
    main_BB15 -> main_SWITCH_INT_12;
    main_BB15 -> main_SWITCH_INT_14;
    main_BB2 -> main_BB2_STMT0;
    main_BB2_END_PLACE -> std_clone_Clone_clone_0;
    main_BB3 -> main_BB3_STMT0;
    main_BB3_END_PLACE -> std_thread_spawn_0;
    main_BB3_STMT0_END -> main_BB3_STMT1;
    main_BB4 -> main_BB4_STMT0;
    main_BB4_END_PLACE -> std_ops_Deref_deref_0;
    main_BB4_STMT0_END -> main_BB4_STMT1;
    main_BB5 -> main_BB5_STMT0;
    main_BB5_END_PLACE -> std_sync_Mutex_T_lock_0;
    main_BB6 -> main_BB6_STMT0;
    main_BB6_END_PLACE -> std_thread_JoinHandle_T_join_0;
    main_BB6_STMT0_END -> main_BB6_STMT1;
    main_BB7 -> main_DROP_7;
    main_BB7 -> main_DROP_UNWIND_7;
    main_BB8 -> main_DROP_8;
    main_BB8 -> main_DROP_UNWIND_8;
    main_BB9 -> main_BB9_STMT0;
    main_BB9_END_PLACE -> main_DROP_9;
    main__closure_0__BB0_END_PLACE -> std_ops_Deref_deref_1;
    main__closure_0__BB1 -> main__closure_0__BB1_STMT0;
    main__closure_0__BB1_END_PLACE -> std_sync_Mutex_T_lock_1;
    main__closure_0__BB2 -> main__closure_0__DROP_2;
    main__closure_0__BB2 -> main__closure_0__DROP_UNWIND_2;
    main__closure_0__BB3 -> main__closure_0__DROP_3;
    main__closure_0__BB4 -> main__closure_0__RETURN;
    main__closure_0__BB5 -> main__closure_0__DROP_5;
    main__closure_0__BB6 -> main__closure_0__UNWIND_6;
    main_BB0_STMT0 -> main_BB0_END_PLACE;
    main_BB2_STMT0 -> main_BB2_END_PLACE;
    main_BB3_STMT0 -> main_BB3_STMT0_END;
    main_BB3_STMT1 -> main_BB3_END_PLACE;
    main_BB4_STMT0 -> main_BB4_STMT0_END;
    main_BB4_STMT1 -> main_BB4_END_PLACE;
    main_BB5_STMT0 -> main_BB5_END_PLACE;
    main_BB6_STMT0 -> main_BB6_STMT0_END;
    main_BB6_STMT1 -> main_BB6_END_PLACE;
    main_BB9_STMT0 -> main_BB9_END_PLACE;
    main_DROP_11 -> MUTEX_0;
    main_DROP_11 -> main_BB15;
    main_DROP_12 -> main_BB13;
    main_DROP_14 -> main_BB12;
    main_DROP_7 -> main_BB8;
    main_DROP_8 -> MUTEX_0;
    main_DROP_8 -> main_BB9;
    main_DROP_9 -> main_BB10;
    main_DROP_UNWIND_7 -> main_BB11;
    main_DROP_UNWIND_8 -> main_BB15;
    main_RETURN -> PROGRAM_END;
    main_SWITCH_INT_12 -> main_BB12;
    main_SWITCH_INT_14 -> main_BB14;
    main_UNWIND_13 -> PROGRAM_PANIC;
    main__closure_0__BB0_STMT0 -> main__closure_0__BB0_END_PLACE;
    main__closure_0__BB1_STMT0 -> main__closure_0__BB1_END_PLACE;
    main__closure_0__DROP_2 -> MUTEX_0;
    main__closure_0__DROP_2 -> main__closure_0__BB3;
    main__closure_0__DROP_3 -> main__closure_0__BB4;
    main__closure_0__DROP_5 -> main__closure_0__BB6;
    main__closure_0__DROP_UNWIND_2 -> main__closure_0__BB5;
    main__closure_0__RETURN -> THREAD_END_0;
    main__closure_0__UNWIND_6 -> PROGRAM_PANIC;
    std_clone_Clone_clone_0 -> main_BB12;
    std_clone_Clone_clone_0 -> main_BB3;
    std_ops_Deref_deref_0 -> main_BB15;
    std_ops_Deref_deref_0 -> main_BB5;
    std_ops_Deref_deref_1 -> main__closure_0__BB1;
    std_ops_Deref_deref_1 -> main__closure_0__BB5;
    std_sync_Arc_T_new_0 -> main_BB2;
    std_sync_Mutex_T_lock_0 -> main_BB15;
    std_sync_Mutex_T_lock_0 -> main_BB6;
    std_sync_Mutex_T_lock_1 -> main__closure_0__BB2;
    std_sync_Mutex_T_lock_1 -> main__closure_0__BB5;
    std_sync_Mutex_T_new_0 -> main_BB1;
    std_thread_JoinHandle_T_join_0 -> main_BB11;
    std_thread_JoinHandle_T_join_0 -> main_BB7;
    std_thread_spawn_0 -> THREAD_START_0;
    std_thread_spawn_0 -> main_BB12;
    std_thread_spawn_0 -> main_BB4;
}
"#;

const MINIMAL_SHARED_MUTEX_LOLA_OUTPUT: &str = r#"PLACE
    MUTEX_0,
    PROGRAM_END,
    PROGRAM_PANIC,
    PROGRAM_START,
    THREAD_END_0,
    THREAD_START_0,
    main_BB0_END_PLACE,
    main_BB1,
    main_BB10,
    main_BB11,
    main_BB12,
    main_BB13,
    main_BB14,
    main_BB15,
    main_BB2,
    main_BB2_END_PLACE,
    main_BB3,
    main_BB3_END_PLACE,
    main_BB3_STMT0_END,
    main_BB4,
    main_BB4_END_PLACE,
    main_BB4_STMT0_END,
    main_BB5,
    main_BB5_END_PLACE,
    main_BB6,
    main_BB6_END_PLACE,
    main_BB6_STMT0_END,
    main_BB7,
    main_BB8,
    main_BB9,
    main_BB9_END_PLACE,
    main__closure_0__BB0_END_PLACE,
    main__closure_0__BB1,
    main__closure_0__BB1_END_PLACE,
    main__closure_0__BB2,
    main__closure_0__BB3,
    main__closure_0__BB4,
    main__closure_0__BB5,
    main__closure_0__BB6;

MARKING
    MUTEX_0 : 1,
    PROGRAM_START : 1,
TRANSITION main_BB0_STMT0
  CONSUME
    PROGRAM_START : 1;
  PRODUCE
    main_BB0_END_PLACE : 1;
TRANSITION main_BB2_STMT0
  CONSUME
    main_BB2 : 1;
  PRODUCE
    main_BB2_END_PLACE : 1;
TRANSITION main_BB3_STMT0
  CONSUME
    main_BB3 : 1;
  PRODUCE
    main_BB3_STMT0_END : 1;
TRANSITION main_BB3_STMT1
  CONSUME
    main_BB3_STMT0_END : 1;
  PRODUCE
    main_BB3_END_PLACE : 1;
TRANSITION main_BB4_STMT0
  CONSUME
    main_BB4 : 1;
  PRODUCE
    main_BB4_STMT0_END : 1;
TRANSITION main_BB4_STMT1
  CONSUME
    main_BB4_STMT0_END : 1;
  PRODUCE
    main_BB4_END_PLACE : 1;
TRANSITION main_BB5_STMT0
  CONSUME
    main_BB5 : 1;
  PRODUCE
    main_BB5_END_PLACE : 1;
TRANSITION main_BB6_STMT0
  CONSUME
    main_BB6 : 1;
  PRODUCE
    main_BB6_STMT0_END : 1;
TRANSITION main_BB6_STMT1
  CONSUME
    main_BB6_STMT0_END : 1;
  PRODUCE
    main_BB6_END_PLACE : 1;
TRANSITION main_BB9_STMT0
  CONSUME
    main_BB9 : 1;
  PRODUCE
    main_BB9_END_PLACE : 1;
TRANSITION main_DROP_11
  CONSUME
    main_BB11 : 1;
  PRODUCE
    MUTEX_0 : 1,
    main_BB15 : 1;
TRANSITION main_DROP_12
  CONSUME
    main_BB12 : 1;
  PRODUCE
    main_BB13 : 1;
TRANSITION main_DROP_14
  CONSUME
    main_BB14 : 1;
  PRODUCE
    main_BB12 : 1;
TRANSITION main_DROP_7
  CONSUME
    main_BB7 : 1;
  PRODUCE
    main_BB8 : 1;
TRANSITION main_DROP_8
  CONSUME
    main_BB8 : 1;
  PRODUCE
    MUTEX_0 : 1,
    main_BB9 : 1;
TRANSITION main_DROP_9
  CONSUME
    main_BB9_END_PLACE : 1;
  PRODUCE
    main_BB10 : 1;
TRANSITION main_DROP_UNWIND_7
  CONSUME
    main_BB7 : 1;
  PRODUCE
    main_BB11 : 1;
TRANSITION main_DROP_UNWIND_8
  CONSUME
    main_BB8 : 1;
  PRODUCE
    main_BB15 : 1;
TRANSITION main_RETURN
  CONSUME
    main_BB10 : 1;
  PRODUCE
    PROGRAM_END : 1;
TRANSITION main_SWITCH_INT_12
  CONSUME
    main_BB15 : 1;
  PRODUCE
    main_BB12 : 1;
TRANSITION main_SWITCH_INT_14
  CONSUME
    main_BB15 : 1;
  PRODUCE
    main_BB14 : 1;
TRANSITION main_UNWIND_13
  CONSUME
    main_BB13 : 1;
  PRODUCE
    PROGRAM_PANIC : 1;
TRANSITION main__closure_0__BB0_STMT0
  CONSUME
    THREAD_START_0 : 1;
  PRODUCE
    main__closure_0__BB0_END_PLACE : 1;
TRANSITION main__closure_0__BB1_STMT0
  CONSUME
    main__closure_0__BB1 : 1;
  PRODUCE
    main__closure_0__BB1_END_PLACE : 1;
TRANSITION main__closure_0__DROP_2
  CONSUME
    main__closure_0__BB2 : 1;
  PRODUCE
    MUTEX_0 : 1,
    main__closure_0__BB3 : 1;
TRANSITION main__closure_0__DROP_3
  CONSUME
    main__closure_0__BB3 : 1;
  PRODUCE
    main__closure_0__BB4 : 1;
TRANSITION main__closure_0__DROP_5
  CONSUME
    main__closure_0__BB5 : 1;
  PRODUCE
    main__closure_0__BB6 : 1;
TRANSITION main__closure_0__DROP_UNWIND_2
  CONSUME
    main__closure_0__BB2 : 1;
  PRODUCE
    main__closure_0__BB5 : 1;
TRANSITION main__closure_0__RETURN
  CONSUME
    main__closure_0__BB4 : 1;
  PRODUCE
    THREAD_END_0 : 1;
TRANSITION main__closure_0__UNWIND_6
  CONSUME
    main__closure_0__BB6 : 1;
  PRODUCE
    PROGRAM_PANIC : 1;
TRANSITION std_clone_Clone_clone_0
  CONSUME
    main_BB2_END_PLACE : 1;
  PRODUCE
    main_BB12 : 1,
    main_BB3 : 1;
TRANSITION std_ops_Deref_deref_0
  CONSUME
    main_BB4_END_PLACE : 1;
  PRODUCE
    main_BB15 : 1,
    main_BB5 : 1;
TRANSITION std_ops_Deref_deref_1
  CONSUME
    main__closure_0__BB0_END_PLACE : 1;
  PRODUCE
    main__closure_0__BB1 : 1,
    main__closure_0__BB5 : 1;
TRANSITION std_sync_Arc_T_new_0
  CONSUME
    main_BB1 : 1;
  PRODUCE
    main_BB2 : 1;
TRANSITION std_sync_Mutex_T_lock_0
  CONSUME
    MUTEX_0 : 1,
    main_BB5_END_PLACE : 1;
  PRODUCE
    main_BB15 : 1,
    main_BB6 : 1;
TRANSITION std_sync_Mutex_T_lock_1
  CONSUME
    MUTEX_0 : 1,
    main__closure_0__BB1_END_PLACE : 1;
  PRODUCE
    main__closure_0__BB2 : 1,
    main__closure_0__BB5 : 1;
TRANSITION std_sync_Mutex_T_new_0
  CONSUME
    main_BB0_END_PLACE : 1;
  PRODUCE
    main_BB1 : 1;
TRANSITION std_thread_JoinHandle_T_join_0
  CONSUME
    THREAD_END_0 : 1,
    main_BB6_END_PLACE : 1;
  PRODUCE
    main_BB11 : 1,
    main_BB7 : 1;
TRANSITION std_thread_spawn_0
  CONSUME
    main_BB3_END_PLACE : 1;
  PRODUCE
    THREAD_START_0 : 1,
    main_BB12 : 1,
    main_BB4 : 1;
"#;

const MINIMAL_SHARED_MUTEX_PNML_OUTPUT: &str = r#"<?xml version="1.0" encoding="utf-8"?>
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
      <place id="main_BB0_END_PLACE">
        <name>
          <text>main_BB0_END_PLACE</text>
        </name>
      </place>
      <place id="main_BB1">
        <name>
          <text>main_BB1</text>
        </name>
      </place>
      <place id="main_BB10">
        <name>
          <text>main_BB10</text>
        </name>
      </place>
      <place id="main_BB11">
        <name>
          <text>main_BB11</text>
        </name>
      </place>
      <place id="main_BB12">
        <name>
          <text>main_BB12</text>
        </name>
      </place>
      <place id="main_BB13">
        <name>
          <text>main_BB13</text>
        </name>
      </place>
      <place id="main_BB14">
        <name>
          <text>main_BB14</text>
        </name>
      </place>
      <place id="main_BB15">
        <name>
          <text>main_BB15</text>
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
      <place id="main_BB3_STMT0_END">
        <name>
          <text>main_BB3_STMT0_END</text>
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
      <place id="main_BB4_STMT0_END">
        <name>
          <text>main_BB4_STMT0_END</text>
        </name>
      </place>
      <place id="main_BB5">
        <name>
          <text>main_BB5</text>
        </name>
      </place>
      <place id="main_BB5_END_PLACE">
        <name>
          <text>main_BB5_END_PLACE</text>
        </name>
      </place>
      <place id="main_BB6">
        <name>
          <text>main_BB6</text>
        </name>
      </place>
      <place id="main_BB6_END_PLACE">
        <name>
          <text>main_BB6_END_PLACE</text>
        </name>
      </place>
      <place id="main_BB6_STMT0_END">
        <name>
          <text>main_BB6_STMT0_END</text>
        </name>
      </place>
      <place id="main_BB7">
        <name>
          <text>main_BB7</text>
        </name>
      </place>
      <place id="main_BB8">
        <name>
          <text>main_BB8</text>
        </name>
      </place>
      <place id="main_BB9">
        <name>
          <text>main_BB9</text>
        </name>
      </place>
      <place id="main_BB9_END_PLACE">
        <name>
          <text>main_BB9_END_PLACE</text>
        </name>
      </place>
      <place id="main__closure_0__BB0_END_PLACE">
        <name>
          <text>main__closure_0__BB0_END_PLACE</text>
        </name>
      </place>
      <place id="main__closure_0__BB1">
        <name>
          <text>main__closure_0__BB1</text>
        </name>
      </place>
      <place id="main__closure_0__BB1_END_PLACE">
        <name>
          <text>main__closure_0__BB1_END_PLACE</text>
        </name>
      </place>
      <place id="main__closure_0__BB2">
        <name>
          <text>main__closure_0__BB2</text>
        </name>
      </place>
      <place id="main__closure_0__BB3">
        <name>
          <text>main__closure_0__BB3</text>
        </name>
      </place>
      <place id="main__closure_0__BB4">
        <name>
          <text>main__closure_0__BB4</text>
        </name>
      </place>
      <place id="main__closure_0__BB5">
        <name>
          <text>main__closure_0__BB5</text>
        </name>
      </place>
      <place id="main__closure_0__BB6">
        <name>
          <text>main__closure_0__BB6</text>
        </name>
      </place>
      <transition id="main_BB0_STMT0">
        <name>
          <text>main_BB0_STMT0</text>
        </name>
      </transition>
      <transition id="main_BB2_STMT0">
        <name>
          <text>main_BB2_STMT0</text>
        </name>
      </transition>
      <transition id="main_BB3_STMT0">
        <name>
          <text>main_BB3_STMT0</text>
        </name>
      </transition>
      <transition id="main_BB3_STMT1">
        <name>
          <text>main_BB3_STMT1</text>
        </name>
      </transition>
      <transition id="main_BB4_STMT0">
        <name>
          <text>main_BB4_STMT0</text>
        </name>
      </transition>
      <transition id="main_BB4_STMT1">
        <name>
          <text>main_BB4_STMT1</text>
        </name>
      </transition>
      <transition id="main_BB5_STMT0">
        <name>
          <text>main_BB5_STMT0</text>
        </name>
      </transition>
      <transition id="main_BB6_STMT0">
        <name>
          <text>main_BB6_STMT0</text>
        </name>
      </transition>
      <transition id="main_BB6_STMT1">
        <name>
          <text>main_BB6_STMT1</text>
        </name>
      </transition>
      <transition id="main_BB9_STMT0">
        <name>
          <text>main_BB9_STMT0</text>
        </name>
      </transition>
      <transition id="main_DROP_11">
        <name>
          <text>main_DROP_11</text>
        </name>
      </transition>
      <transition id="main_DROP_12">
        <name>
          <text>main_DROP_12</text>
        </name>
      </transition>
      <transition id="main_DROP_14">
        <name>
          <text>main_DROP_14</text>
        </name>
      </transition>
      <transition id="main_DROP_7">
        <name>
          <text>main_DROP_7</text>
        </name>
      </transition>
      <transition id="main_DROP_8">
        <name>
          <text>main_DROP_8</text>
        </name>
      </transition>
      <transition id="main_DROP_9">
        <name>
          <text>main_DROP_9</text>
        </name>
      </transition>
      <transition id="main_DROP_UNWIND_7">
        <name>
          <text>main_DROP_UNWIND_7</text>
        </name>
      </transition>
      <transition id="main_DROP_UNWIND_8">
        <name>
          <text>main_DROP_UNWIND_8</text>
        </name>
      </transition>
      <transition id="main_RETURN">
        <name>
          <text>main_RETURN</text>
        </name>
      </transition>
      <transition id="main_SWITCH_INT_12">
        <name>
          <text>main_SWITCH_INT_12</text>
        </name>
      </transition>
      <transition id="main_SWITCH_INT_14">
        <name>
          <text>main_SWITCH_INT_14</text>
        </name>
      </transition>
      <transition id="main_UNWIND_13">
        <name>
          <text>main_UNWIND_13</text>
        </name>
      </transition>
      <transition id="main__closure_0__BB0_STMT0">
        <name>
          <text>main__closure_0__BB0_STMT0</text>
        </name>
      </transition>
      <transition id="main__closure_0__BB1_STMT0">
        <name>
          <text>main__closure_0__BB1_STMT0</text>
        </name>
      </transition>
      <transition id="main__closure_0__DROP_2">
        <name>
          <text>main__closure_0__DROP_2</text>
        </name>
      </transition>
      <transition id="main__closure_0__DROP_3">
        <name>
          <text>main__closure_0__DROP_3</text>
        </name>
      </transition>
      <transition id="main__closure_0__DROP_5">
        <name>
          <text>main__closure_0__DROP_5</text>
        </name>
      </transition>
      <transition id="main__closure_0__DROP_UNWIND_2">
        <name>
          <text>main__closure_0__DROP_UNWIND_2</text>
        </name>
      </transition>
      <transition id="main__closure_0__RETURN">
        <name>
          <text>main__closure_0__RETURN</text>
        </name>
      </transition>
      <transition id="main__closure_0__UNWIND_6">
        <name>
          <text>main__closure_0__UNWIND_6</text>
        </name>
      </transition>
      <transition id="std_clone_Clone_clone_0">
        <name>
          <text>std_clone_Clone_clone_0</text>
        </name>
      </transition>
      <transition id="std_ops_Deref_deref_0">
        <name>
          <text>std_ops_Deref_deref_0</text>
        </name>
      </transition>
      <transition id="std_ops_Deref_deref_1">
        <name>
          <text>std_ops_Deref_deref_1</text>
        </name>
      </transition>
      <transition id="std_sync_Arc_T_new_0">
        <name>
          <text>std_sync_Arc_T_new_0</text>
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
      <arc source="PROGRAM_START" target="main_BB0_STMT0" id="(PROGRAM_START, main_BB0_STMT0)">
        <name>
          <text>(PROGRAM_START, main_BB0_STMT0)</text>
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
      <arc source="THREAD_START_0" target="main__closure_0__BB0_STMT0" id="(THREAD_START_0, main__closure_0__BB0_STMT0)">
        <name>
          <text>(THREAD_START_0, main__closure_0__BB0_STMT0)</text>
        </name>
        <inscription>
          <text>1</text>
        </inscription>
      </arc>
      <arc source="main_BB0_END_PLACE" target="std_sync_Mutex_T_new_0" id="(main_BB0_END_PLACE, std_sync_Mutex_T_new_0)">
        <name>
          <text>(main_BB0_END_PLACE, std_sync_Mutex_T_new_0)</text>
        </name>
        <inscription>
          <text>1</text>
        </inscription>
      </arc>
      <arc source="main_BB1" target="std_sync_Arc_T_new_0" id="(main_BB1, std_sync_Arc_T_new_0)">
        <name>
          <text>(main_BB1, std_sync_Arc_T_new_0)</text>
        </name>
        <inscription>
          <text>1</text>
        </inscription>
      </arc>
      <arc source="main_BB10" target="main_RETURN" id="(main_BB10, main_RETURN)">
        <name>
          <text>(main_BB10, main_RETURN)</text>
        </name>
        <inscription>
          <text>1</text>
        </inscription>
      </arc>
      <arc source="main_BB11" target="main_DROP_11" id="(main_BB11, main_DROP_11)">
        <name>
          <text>(main_BB11, main_DROP_11)</text>
        </name>
        <inscription>
          <text>1</text>
        </inscription>
      </arc>
      <arc source="main_BB12" target="main_DROP_12" id="(main_BB12, main_DROP_12)">
        <name>
          <text>(main_BB12, main_DROP_12)</text>
        </name>
        <inscription>
          <text>1</text>
        </inscription>
      </arc>
      <arc source="main_BB13" target="main_UNWIND_13" id="(main_BB13, main_UNWIND_13)">
        <name>
          <text>(main_BB13, main_UNWIND_13)</text>
        </name>
        <inscription>
          <text>1</text>
        </inscription>
      </arc>
      <arc source="main_BB14" target="main_DROP_14" id="(main_BB14, main_DROP_14)">
        <name>
          <text>(main_BB14, main_DROP_14)</text>
        </name>
        <inscription>
          <text>1</text>
        </inscription>
      </arc>
      <arc source="main_BB15" target="main_SWITCH_INT_12" id="(main_BB15, main_SWITCH_INT_12)">
        <name>
          <text>(main_BB15, main_SWITCH_INT_12)</text>
        </name>
        <inscription>
          <text>1</text>
        </inscription>
      </arc>
      <arc source="main_BB15" target="main_SWITCH_INT_14" id="(main_BB15, main_SWITCH_INT_14)">
        <name>
          <text>(main_BB15, main_SWITCH_INT_14)</text>
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
      <arc source="main_BB2_END_PLACE" target="std_clone_Clone_clone_0" id="(main_BB2_END_PLACE, std_clone_Clone_clone_0)">
        <name>
          <text>(main_BB2_END_PLACE, std_clone_Clone_clone_0)</text>
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
      <arc source="main_BB3_END_PLACE" target="std_thread_spawn_0" id="(main_BB3_END_PLACE, std_thread_spawn_0)">
        <name>
          <text>(main_BB3_END_PLACE, std_thread_spawn_0)</text>
        </name>
        <inscription>
          <text>1</text>
        </inscription>
      </arc>
      <arc source="main_BB3_STMT0_END" target="main_BB3_STMT1" id="(main_BB3_STMT0_END, main_BB3_STMT1)">
        <name>
          <text>(main_BB3_STMT0_END, main_BB3_STMT1)</text>
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
      <arc source="main_BB4_END_PLACE" target="std_ops_Deref_deref_0" id="(main_BB4_END_PLACE, std_ops_Deref_deref_0)">
        <name>
          <text>(main_BB4_END_PLACE, std_ops_Deref_deref_0)</text>
        </name>
        <inscription>
          <text>1</text>
        </inscription>
      </arc>
      <arc source="main_BB4_STMT0_END" target="main_BB4_STMT1" id="(main_BB4_STMT0_END, main_BB4_STMT1)">
        <name>
          <text>(main_BB4_STMT0_END, main_BB4_STMT1)</text>
        </name>
        <inscription>
          <text>1</text>
        </inscription>
      </arc>
      <arc source="main_BB5" target="main_BB5_STMT0" id="(main_BB5, main_BB5_STMT0)">
        <name>
          <text>(main_BB5, main_BB5_STMT0)</text>
        </name>
        <inscription>
          <text>1</text>
        </inscription>
      </arc>
      <arc source="main_BB5_END_PLACE" target="std_sync_Mutex_T_lock_0" id="(main_BB5_END_PLACE, std_sync_Mutex_T_lock_0)">
        <name>
          <text>(main_BB5_END_PLACE, std_sync_Mutex_T_lock_0)</text>
        </name>
        <inscription>
          <text>1</text>
        </inscription>
      </arc>
      <arc source="main_BB6" target="main_BB6_STMT0" id="(main_BB6, main_BB6_STMT0)">
        <name>
          <text>(main_BB6, main_BB6_STMT0)</text>
        </name>
        <inscription>
          <text>1</text>
        </inscription>
      </arc>
      <arc source="main_BB6_END_PLACE" target="std_thread_JoinHandle_T_join_0" id="(main_BB6_END_PLACE, std_thread_JoinHandle_T_join_0)">
        <name>
          <text>(main_BB6_END_PLACE, std_thread_JoinHandle_T_join_0)</text>
        </name>
        <inscription>
          <text>1</text>
        </inscription>
      </arc>
      <arc source="main_BB6_STMT0_END" target="main_BB6_STMT1" id="(main_BB6_STMT0_END, main_BB6_STMT1)">
        <name>
          <text>(main_BB6_STMT0_END, main_BB6_STMT1)</text>
        </name>
        <inscription>
          <text>1</text>
        </inscription>
      </arc>
      <arc source="main_BB7" target="main_DROP_7" id="(main_BB7, main_DROP_7)">
        <name>
          <text>(main_BB7, main_DROP_7)</text>
        </name>
        <inscription>
          <text>1</text>
        </inscription>
      </arc>
      <arc source="main_BB7" target="main_DROP_UNWIND_7" id="(main_BB7, main_DROP_UNWIND_7)">
        <name>
          <text>(main_BB7, main_DROP_UNWIND_7)</text>
        </name>
        <inscription>
          <text>1</text>
        </inscription>
      </arc>
      <arc source="main_BB8" target="main_DROP_8" id="(main_BB8, main_DROP_8)">
        <name>
          <text>(main_BB8, main_DROP_8)</text>
        </name>
        <inscription>
          <text>1</text>
        </inscription>
      </arc>
      <arc source="main_BB8" target="main_DROP_UNWIND_8" id="(main_BB8, main_DROP_UNWIND_8)">
        <name>
          <text>(main_BB8, main_DROP_UNWIND_8)</text>
        </name>
        <inscription>
          <text>1</text>
        </inscription>
      </arc>
      <arc source="main_BB9" target="main_BB9_STMT0" id="(main_BB9, main_BB9_STMT0)">
        <name>
          <text>(main_BB9, main_BB9_STMT0)</text>
        </name>
        <inscription>
          <text>1</text>
        </inscription>
      </arc>
      <arc source="main_BB9_END_PLACE" target="main_DROP_9" id="(main_BB9_END_PLACE, main_DROP_9)">
        <name>
          <text>(main_BB9_END_PLACE, main_DROP_9)</text>
        </name>
        <inscription>
          <text>1</text>
        </inscription>
      </arc>
      <arc source="main__closure_0__BB0_END_PLACE" target="std_ops_Deref_deref_1" id="(main__closure_0__BB0_END_PLACE, std_ops_Deref_deref_1)">
        <name>
          <text>(main__closure_0__BB0_END_PLACE, std_ops_Deref_deref_1)</text>
        </name>
        <inscription>
          <text>1</text>
        </inscription>
      </arc>
      <arc source="main__closure_0__BB1" target="main__closure_0__BB1_STMT0" id="(main__closure_0__BB1, main__closure_0__BB1_STMT0)">
        <name>
          <text>(main__closure_0__BB1, main__closure_0__BB1_STMT0)</text>
        </name>
        <inscription>
          <text>1</text>
        </inscription>
      </arc>
      <arc source="main__closure_0__BB1_END_PLACE" target="std_sync_Mutex_T_lock_1" id="(main__closure_0__BB1_END_PLACE, std_sync_Mutex_T_lock_1)">
        <name>
          <text>(main__closure_0__BB1_END_PLACE, std_sync_Mutex_T_lock_1)</text>
        </name>
        <inscription>
          <text>1</text>
        </inscription>
      </arc>
      <arc source="main__closure_0__BB2" target="main__closure_0__DROP_2" id="(main__closure_0__BB2, main__closure_0__DROP_2)">
        <name>
          <text>(main__closure_0__BB2, main__closure_0__DROP_2)</text>
        </name>
        <inscription>
          <text>1</text>
        </inscription>
      </arc>
      <arc source="main__closure_0__BB2" target="main__closure_0__DROP_UNWIND_2" id="(main__closure_0__BB2, main__closure_0__DROP_UNWIND_2)">
        <name>
          <text>(main__closure_0__BB2, main__closure_0__DROP_UNWIND_2)</text>
        </name>
        <inscription>
          <text>1</text>
        </inscription>
      </arc>
      <arc source="main__closure_0__BB3" target="main__closure_0__DROP_3" id="(main__closure_0__BB3, main__closure_0__DROP_3)">
        <name>
          <text>(main__closure_0__BB3, main__closure_0__DROP_3)</text>
        </name>
        <inscription>
          <text>1</text>
        </inscription>
      </arc>
      <arc source="main__closure_0__BB4" target="main__closure_0__RETURN" id="(main__closure_0__BB4, main__closure_0__RETURN)">
        <name>
          <text>(main__closure_0__BB4, main__closure_0__RETURN)</text>
        </name>
        <inscription>
          <text>1</text>
        </inscription>
      </arc>
      <arc source="main__closure_0__BB5" target="main__closure_0__DROP_5" id="(main__closure_0__BB5, main__closure_0__DROP_5)">
        <name>
          <text>(main__closure_0__BB5, main__closure_0__DROP_5)</text>
        </name>
        <inscription>
          <text>1</text>
        </inscription>
      </arc>
      <arc source="main__closure_0__BB6" target="main__closure_0__UNWIND_6" id="(main__closure_0__BB6, main__closure_0__UNWIND_6)">
        <name>
          <text>(main__closure_0__BB6, main__closure_0__UNWIND_6)</text>
        </name>
        <inscription>
          <text>1</text>
        </inscription>
      </arc>
      <arc source="main_BB0_STMT0" target="main_BB0_END_PLACE" id="(main_BB0_STMT0, main_BB0_END_PLACE)">
        <name>
          <text>(main_BB0_STMT0, main_BB0_END_PLACE)</text>
        </name>
        <inscription>
          <text>1</text>
        </inscription>
      </arc>
      <arc source="main_BB2_STMT0" target="main_BB2_END_PLACE" id="(main_BB2_STMT0, main_BB2_END_PLACE)">
        <name>
          <text>(main_BB2_STMT0, main_BB2_END_PLACE)</text>
        </name>
        <inscription>
          <text>1</text>
        </inscription>
      </arc>
      <arc source="main_BB3_STMT0" target="main_BB3_STMT0_END" id="(main_BB3_STMT0, main_BB3_STMT0_END)">
        <name>
          <text>(main_BB3_STMT0, main_BB3_STMT0_END)</text>
        </name>
        <inscription>
          <text>1</text>
        </inscription>
      </arc>
      <arc source="main_BB3_STMT1" target="main_BB3_END_PLACE" id="(main_BB3_STMT1, main_BB3_END_PLACE)">
        <name>
          <text>(main_BB3_STMT1, main_BB3_END_PLACE)</text>
        </name>
        <inscription>
          <text>1</text>
        </inscription>
      </arc>
      <arc source="main_BB4_STMT0" target="main_BB4_STMT0_END" id="(main_BB4_STMT0, main_BB4_STMT0_END)">
        <name>
          <text>(main_BB4_STMT0, main_BB4_STMT0_END)</text>
        </name>
        <inscription>
          <text>1</text>
        </inscription>
      </arc>
      <arc source="main_BB4_STMT1" target="main_BB4_END_PLACE" id="(main_BB4_STMT1, main_BB4_END_PLACE)">
        <name>
          <text>(main_BB4_STMT1, main_BB4_END_PLACE)</text>
        </name>
        <inscription>
          <text>1</text>
        </inscription>
      </arc>
      <arc source="main_BB5_STMT0" target="main_BB5_END_PLACE" id="(main_BB5_STMT0, main_BB5_END_PLACE)">
        <name>
          <text>(main_BB5_STMT0, main_BB5_END_PLACE)</text>
        </name>
        <inscription>
          <text>1</text>
        </inscription>
      </arc>
      <arc source="main_BB6_STMT0" target="main_BB6_STMT0_END" id="(main_BB6_STMT0, main_BB6_STMT0_END)">
        <name>
          <text>(main_BB6_STMT0, main_BB6_STMT0_END)</text>
        </name>
        <inscription>
          <text>1</text>
        </inscription>
      </arc>
      <arc source="main_BB6_STMT1" target="main_BB6_END_PLACE" id="(main_BB6_STMT1, main_BB6_END_PLACE)">
        <name>
          <text>(main_BB6_STMT1, main_BB6_END_PLACE)</text>
        </name>
        <inscription>
          <text>1</text>
        </inscription>
      </arc>
      <arc source="main_BB9_STMT0" target="main_BB9_END_PLACE" id="(main_BB9_STMT0, main_BB9_END_PLACE)">
        <name>
          <text>(main_BB9_STMT0, main_BB9_END_PLACE)</text>
        </name>
        <inscription>
          <text>1</text>
        </inscription>
      </arc>
      <arc source="main_DROP_11" target="MUTEX_0" id="(main_DROP_11, MUTEX_0)">
        <name>
          <text>(main_DROP_11, MUTEX_0)</text>
        </name>
        <inscription>
          <text>1</text>
        </inscription>
      </arc>
      <arc source="main_DROP_11" target="main_BB15" id="(main_DROP_11, main_BB15)">
        <name>
          <text>(main_DROP_11, main_BB15)</text>
        </name>
        <inscription>
          <text>1</text>
        </inscription>
      </arc>
      <arc source="main_DROP_12" target="main_BB13" id="(main_DROP_12, main_BB13)">
        <name>
          <text>(main_DROP_12, main_BB13)</text>
        </name>
        <inscription>
          <text>1</text>
        </inscription>
      </arc>
      <arc source="main_DROP_14" target="main_BB12" id="(main_DROP_14, main_BB12)">
        <name>
          <text>(main_DROP_14, main_BB12)</text>
        </name>
        <inscription>
          <text>1</text>
        </inscription>
      </arc>
      <arc source="main_DROP_7" target="main_BB8" id="(main_DROP_7, main_BB8)">
        <name>
          <text>(main_DROP_7, main_BB8)</text>
        </name>
        <inscription>
          <text>1</text>
        </inscription>
      </arc>
      <arc source="main_DROP_8" target="MUTEX_0" id="(main_DROP_8, MUTEX_0)">
        <name>
          <text>(main_DROP_8, MUTEX_0)</text>
        </name>
        <inscription>
          <text>1</text>
        </inscription>
      </arc>
      <arc source="main_DROP_8" target="main_BB9" id="(main_DROP_8, main_BB9)">
        <name>
          <text>(main_DROP_8, main_BB9)</text>
        </name>
        <inscription>
          <text>1</text>
        </inscription>
      </arc>
      <arc source="main_DROP_9" target="main_BB10" id="(main_DROP_9, main_BB10)">
        <name>
          <text>(main_DROP_9, main_BB10)</text>
        </name>
        <inscription>
          <text>1</text>
        </inscription>
      </arc>
      <arc source="main_DROP_UNWIND_7" target="main_BB11" id="(main_DROP_UNWIND_7, main_BB11)">
        <name>
          <text>(main_DROP_UNWIND_7, main_BB11)</text>
        </name>
        <inscription>
          <text>1</text>
        </inscription>
      </arc>
      <arc source="main_DROP_UNWIND_8" target="main_BB15" id="(main_DROP_UNWIND_8, main_BB15)">
        <name>
          <text>(main_DROP_UNWIND_8, main_BB15)</text>
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
      <arc source="main_SWITCH_INT_12" target="main_BB12" id="(main_SWITCH_INT_12, main_BB12)">
        <name>
          <text>(main_SWITCH_INT_12, main_BB12)</text>
        </name>
        <inscription>
          <text>1</text>
        </inscription>
      </arc>
      <arc source="main_SWITCH_INT_14" target="main_BB14" id="(main_SWITCH_INT_14, main_BB14)">
        <name>
          <text>(main_SWITCH_INT_14, main_BB14)</text>
        </name>
        <inscription>
          <text>1</text>
        </inscription>
      </arc>
      <arc source="main_UNWIND_13" target="PROGRAM_PANIC" id="(main_UNWIND_13, PROGRAM_PANIC)">
        <name>
          <text>(main_UNWIND_13, PROGRAM_PANIC)</text>
        </name>
        <inscription>
          <text>1</text>
        </inscription>
      </arc>
      <arc source="main__closure_0__BB0_STMT0" target="main__closure_0__BB0_END_PLACE" id="(main__closure_0__BB0_STMT0, main__closure_0__BB0_END_PLACE)">
        <name>
          <text>(main__closure_0__BB0_STMT0, main__closure_0__BB0_END_PLACE)</text>
        </name>
        <inscription>
          <text>1</text>
        </inscription>
      </arc>
      <arc source="main__closure_0__BB1_STMT0" target="main__closure_0__BB1_END_PLACE" id="(main__closure_0__BB1_STMT0, main__closure_0__BB1_END_PLACE)">
        <name>
          <text>(main__closure_0__BB1_STMT0, main__closure_0__BB1_END_PLACE)</text>
        </name>
        <inscription>
          <text>1</text>
        </inscription>
      </arc>
      <arc source="main__closure_0__DROP_2" target="MUTEX_0" id="(main__closure_0__DROP_2, MUTEX_0)">
        <name>
          <text>(main__closure_0__DROP_2, MUTEX_0)</text>
        </name>
        <inscription>
          <text>1</text>
        </inscription>
      </arc>
      <arc source="main__closure_0__DROP_2" target="main__closure_0__BB3" id="(main__closure_0__DROP_2, main__closure_0__BB3)">
        <name>
          <text>(main__closure_0__DROP_2, main__closure_0__BB3)</text>
        </name>
        <inscription>
          <text>1</text>
        </inscription>
      </arc>
      <arc source="main__closure_0__DROP_3" target="main__closure_0__BB4" id="(main__closure_0__DROP_3, main__closure_0__BB4)">
        <name>
          <text>(main__closure_0__DROP_3, main__closure_0__BB4)</text>
        </name>
        <inscription>
          <text>1</text>
        </inscription>
      </arc>
      <arc source="main__closure_0__DROP_5" target="main__closure_0__BB6" id="(main__closure_0__DROP_5, main__closure_0__BB6)">
        <name>
          <text>(main__closure_0__DROP_5, main__closure_0__BB6)</text>
        </name>
        <inscription>
          <text>1</text>
        </inscription>
      </arc>
      <arc source="main__closure_0__DROP_UNWIND_2" target="main__closure_0__BB5" id="(main__closure_0__DROP_UNWIND_2, main__closure_0__BB5)">
        <name>
          <text>(main__closure_0__DROP_UNWIND_2, main__closure_0__BB5)</text>
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
      <arc source="main__closure_0__UNWIND_6" target="PROGRAM_PANIC" id="(main__closure_0__UNWIND_6, PROGRAM_PANIC)">
        <name>
          <text>(main__closure_0__UNWIND_6, PROGRAM_PANIC)</text>
        </name>
        <inscription>
          <text>1</text>
        </inscription>
      </arc>
      <arc source="std_clone_Clone_clone_0" target="main_BB12" id="(std_clone_Clone_clone_0, main_BB12)">
        <name>
          <text>(std_clone_Clone_clone_0, main_BB12)</text>
        </name>
        <inscription>
          <text>1</text>
        </inscription>
      </arc>
      <arc source="std_clone_Clone_clone_0" target="main_BB3" id="(std_clone_Clone_clone_0, main_BB3)">
        <name>
          <text>(std_clone_Clone_clone_0, main_BB3)</text>
        </name>
        <inscription>
          <text>1</text>
        </inscription>
      </arc>
      <arc source="std_ops_Deref_deref_0" target="main_BB15" id="(std_ops_Deref_deref_0, main_BB15)">
        <name>
          <text>(std_ops_Deref_deref_0, main_BB15)</text>
        </name>
        <inscription>
          <text>1</text>
        </inscription>
      </arc>
      <arc source="std_ops_Deref_deref_0" target="main_BB5" id="(std_ops_Deref_deref_0, main_BB5)">
        <name>
          <text>(std_ops_Deref_deref_0, main_BB5)</text>
        </name>
        <inscription>
          <text>1</text>
        </inscription>
      </arc>
      <arc source="std_ops_Deref_deref_1" target="main__closure_0__BB1" id="(std_ops_Deref_deref_1, main__closure_0__BB1)">
        <name>
          <text>(std_ops_Deref_deref_1, main__closure_0__BB1)</text>
        </name>
        <inscription>
          <text>1</text>
        </inscription>
      </arc>
      <arc source="std_ops_Deref_deref_1" target="main__closure_0__BB5" id="(std_ops_Deref_deref_1, main__closure_0__BB5)">
        <name>
          <text>(std_ops_Deref_deref_1, main__closure_0__BB5)</text>
        </name>
        <inscription>
          <text>1</text>
        </inscription>
      </arc>
      <arc source="std_sync_Arc_T_new_0" target="main_BB2" id="(std_sync_Arc_T_new_0, main_BB2)">
        <name>
          <text>(std_sync_Arc_T_new_0, main_BB2)</text>
        </name>
        <inscription>
          <text>1</text>
        </inscription>
      </arc>
      <arc source="std_sync_Mutex_T_lock_0" target="main_BB15" id="(std_sync_Mutex_T_lock_0, main_BB15)">
        <name>
          <text>(std_sync_Mutex_T_lock_0, main_BB15)</text>
        </name>
        <inscription>
          <text>1</text>
        </inscription>
      </arc>
      <arc source="std_sync_Mutex_T_lock_0" target="main_BB6" id="(std_sync_Mutex_T_lock_0, main_BB6)">
        <name>
          <text>(std_sync_Mutex_T_lock_0, main_BB6)</text>
        </name>
        <inscription>
          <text>1</text>
        </inscription>
      </arc>
      <arc source="std_sync_Mutex_T_lock_1" target="main__closure_0__BB2" id="(std_sync_Mutex_T_lock_1, main__closure_0__BB2)">
        <name>
          <text>(std_sync_Mutex_T_lock_1, main__closure_0__BB2)</text>
        </name>
        <inscription>
          <text>1</text>
        </inscription>
      </arc>
      <arc source="std_sync_Mutex_T_lock_1" target="main__closure_0__BB5" id="(std_sync_Mutex_T_lock_1, main__closure_0__BB5)">
        <name>
          <text>(std_sync_Mutex_T_lock_1, main__closure_0__BB5)</text>
        </name>
        <inscription>
          <text>1</text>
        </inscription>
      </arc>
      <arc source="std_sync_Mutex_T_new_0" target="main_BB1" id="(std_sync_Mutex_T_new_0, main_BB1)">
        <name>
          <text>(std_sync_Mutex_T_new_0, main_BB1)</text>
        </name>
        <inscription>
          <text>1</text>
        </inscription>
      </arc>
      <arc source="std_thread_JoinHandle_T_join_0" target="main_BB11" id="(std_thread_JoinHandle_T_join_0, main_BB11)">
        <name>
          <text>(std_thread_JoinHandle_T_join_0, main_BB11)</text>
        </name>
        <inscription>
          <text>1</text>
        </inscription>
      </arc>
      <arc source="std_thread_JoinHandle_T_join_0" target="main_BB7" id="(std_thread_JoinHandle_T_join_0, main_BB7)">
        <name>
          <text>(std_thread_JoinHandle_T_join_0, main_BB7)</text>
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
      <arc source="std_thread_spawn_0" target="main_BB12" id="(std_thread_spawn_0, main_BB12)">
        <name>
          <text>(std_thread_spawn_0, main_BB12)</text>
        </name>
        <inscription>
          <text>1</text>
        </inscription>
      </arc>
      <arc source="std_thread_spawn_0" target="main_BB4" id="(std_thread_spawn_0, main_BB4)">
        <name>
          <text>(std_thread_spawn_0, main_BB4)</text>
        </name>
        <inscription>
          <text>1</text>
        </inscription>
      </arc>
    </page>
  </net>
</pnml>"#;

#[test]
fn minimal_shared_mutex_generates_correct_dot_output_file() {
    assert_output_file(
        "./tests/sample_programs/minimal_shared_mutex.rs",
        "dot",
        "./net.dot",
        MINIMAL_SHARED_MUTEX_DOT_OUTPUT,
    );
}

#[test]
fn minimal_shared_mutex_generates_correct_lola_output_file() {
    assert_output_file(
        "./tests/sample_programs/minimal_shared_mutex.rs",
        "lola",
        "./net.lola",
        MINIMAL_SHARED_MUTEX_LOLA_OUTPUT,
    );
}

#[test]
fn minimal_shared_mutex_generates_correct_pnml_output_file() {
    assert_output_file(
        "./tests/sample_programs/minimal_shared_mutex.rs",
        "pnml",
        "./net.pnml",
        MINIMAL_SHARED_MUTEX_PNML_OUTPUT,
    );
}
