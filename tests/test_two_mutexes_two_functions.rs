mod common;
use common::assert_output_file;

const TWO_MUTEXES_TWO_FUNCTIONS_DOT_OUTPUT: &str = r#"digraph petrinet {
    MUTEX_0 [shape="circle" xlabel="MUTEX_0" label="•"];
    MUTEX_1 [shape="circle" xlabel="MUTEX_1" label="•"];
    PROGRAM_END [shape="circle" xlabel="PROGRAM_END" label=""];
    PROGRAM_PANIC [shape="circle" xlabel="PROGRAM_PANIC" label=""];
    PROGRAM_START [shape="circle" xlabel="PROGRAM_START" label="•"];
    first_deadlock_BASIC_BLOCK_1 [shape="circle" xlabel="first_deadlock_BASIC_BLOCK_1" label=""];
    first_deadlock_BASIC_BLOCK_2 [shape="circle" xlabel="first_deadlock_BASIC_BLOCK_2" label=""];
    first_deadlock_BASIC_BLOCK_3 [shape="circle" xlabel="first_deadlock_BASIC_BLOCK_3" label=""];
    first_deadlock_BASIC_BLOCK_4 [shape="circle" xlabel="first_deadlock_BASIC_BLOCK_4" label=""];
    first_deadlock_BASIC_BLOCK_5 [shape="circle" xlabel="first_deadlock_BASIC_BLOCK_5" label=""];
    first_deadlock_BASIC_BLOCK_6 [shape="circle" xlabel="first_deadlock_BASIC_BLOCK_6" label=""];
    first_deadlock_BASIC_BLOCK_7 [shape="circle" xlabel="first_deadlock_BASIC_BLOCK_7" label=""];
    first_deadlock_BASIC_BLOCK_END_PLACE_1 [shape="circle" xlabel="first_deadlock_BASIC_BLOCK_END_PLACE_1" label=""];
    first_deadlock_BASIC_BLOCK_END_PLACE_2 [shape="circle" xlabel="first_deadlock_BASIC_BLOCK_END_PLACE_2" label=""];
    main_BASIC_BLOCK_1 [shape="circle" xlabel="main_BASIC_BLOCK_1" label=""];
    main_BASIC_BLOCK_2 [shape="circle" xlabel="main_BASIC_BLOCK_2" label=""];
    second_deadlock_BASIC_BLOCK_1 [shape="circle" xlabel="second_deadlock_BASIC_BLOCK_1" label=""];
    second_deadlock_BASIC_BLOCK_2 [shape="circle" xlabel="second_deadlock_BASIC_BLOCK_2" label=""];
    second_deadlock_BASIC_BLOCK_3 [shape="circle" xlabel="second_deadlock_BASIC_BLOCK_3" label=""];
    second_deadlock_BASIC_BLOCK_4 [shape="circle" xlabel="second_deadlock_BASIC_BLOCK_4" label=""];
    second_deadlock_BASIC_BLOCK_5 [shape="circle" xlabel="second_deadlock_BASIC_BLOCK_5" label=""];
    second_deadlock_BASIC_BLOCK_6 [shape="circle" xlabel="second_deadlock_BASIC_BLOCK_6" label=""];
    second_deadlock_BASIC_BLOCK_7 [shape="circle" xlabel="second_deadlock_BASIC_BLOCK_7" label=""];
    second_deadlock_BASIC_BLOCK_END_PLACE_1 [shape="circle" xlabel="second_deadlock_BASIC_BLOCK_END_PLACE_1" label=""];
    second_deadlock_BASIC_BLOCK_END_PLACE_2 [shape="circle" xlabel="second_deadlock_BASIC_BLOCK_END_PLACE_2" label=""];
    first_deadlock_BLOCK_1_STATEMENT_0 [shape="box" xlabel="first_deadlock_BLOCK_1_STATEMENT_0" label=""];
    first_deadlock_BLOCK_2_STATEMENT_0 [shape="box" xlabel="first_deadlock_BLOCK_2_STATEMENT_0" label=""];
    first_deadlock_DROP_3 [shape="box" xlabel="first_deadlock_DROP_3" label=""];
    first_deadlock_DROP_4 [shape="box" xlabel="first_deadlock_DROP_4" label=""];
    first_deadlock_DROP_6 [shape="box" xlabel="first_deadlock_DROP_6" label=""];
    first_deadlock_DROP_UNWIND_3 [shape="box" xlabel="first_deadlock_DROP_UNWIND_3" label=""];
    first_deadlock_RETURN [shape="box" xlabel="first_deadlock_RETURN" label=""];
    first_deadlock_UNWIND_7 [shape="box" xlabel="first_deadlock_UNWIND_7" label=""];
    main_RETURN [shape="box" xlabel="main_RETURN" label=""];
    second_deadlock_BLOCK_1_STATEMENT_0 [shape="box" xlabel="second_deadlock_BLOCK_1_STATEMENT_0" label=""];
    second_deadlock_BLOCK_2_STATEMENT_0 [shape="box" xlabel="second_deadlock_BLOCK_2_STATEMENT_0" label=""];
    second_deadlock_DROP_3 [shape="box" xlabel="second_deadlock_DROP_3" label=""];
    second_deadlock_DROP_4 [shape="box" xlabel="second_deadlock_DROP_4" label=""];
    second_deadlock_DROP_6 [shape="box" xlabel="second_deadlock_DROP_6" label=""];
    second_deadlock_DROP_UNWIND_3 [shape="box" xlabel="second_deadlock_DROP_UNWIND_3" label=""];
    second_deadlock_RETURN [shape="box" xlabel="second_deadlock_RETURN" label=""];
    second_deadlock_UNWIND_7 [shape="box" xlabel="second_deadlock_UNWIND_7" label=""];
    std_sync_Mutex_T_lock_0 [shape="box" xlabel="std_sync_Mutex_T_lock_0" label=""];
    std_sync_Mutex_T_lock_1 [shape="box" xlabel="std_sync_Mutex_T_lock_1" label=""];
    std_sync_Mutex_T_lock_2 [shape="box" xlabel="std_sync_Mutex_T_lock_2" label=""];
    std_sync_Mutex_T_lock_3 [shape="box" xlabel="std_sync_Mutex_T_lock_3" label=""];
    std_sync_Mutex_T_new_0 [shape="box" xlabel="std_sync_Mutex_T_new_0" label=""];
    std_sync_Mutex_T_new_1 [shape="box" xlabel="std_sync_Mutex_T_new_1" label=""];
    MUTEX_0 -> std_sync_Mutex_T_lock_0;
    MUTEX_0 -> std_sync_Mutex_T_lock_1;
    MUTEX_1 -> std_sync_Mutex_T_lock_2;
    MUTEX_1 -> std_sync_Mutex_T_lock_3;
    PROGRAM_START -> std_sync_Mutex_T_new_0;
    first_deadlock_BASIC_BLOCK_1 -> first_deadlock_BLOCK_1_STATEMENT_0;
    first_deadlock_BASIC_BLOCK_2 -> first_deadlock_BLOCK_2_STATEMENT_0;
    first_deadlock_BASIC_BLOCK_3 -> first_deadlock_DROP_3;
    first_deadlock_BASIC_BLOCK_3 -> first_deadlock_DROP_UNWIND_3;
    first_deadlock_BASIC_BLOCK_4 -> first_deadlock_DROP_4;
    first_deadlock_BASIC_BLOCK_5 -> first_deadlock_RETURN;
    first_deadlock_BASIC_BLOCK_6 -> first_deadlock_DROP_6;
    first_deadlock_BASIC_BLOCK_7 -> first_deadlock_UNWIND_7;
    first_deadlock_BASIC_BLOCK_END_PLACE_1 -> std_sync_Mutex_T_lock_0;
    first_deadlock_BASIC_BLOCK_END_PLACE_2 -> std_sync_Mutex_T_lock_1;
    main_BASIC_BLOCK_1 -> std_sync_Mutex_T_new_1;
    main_BASIC_BLOCK_2 -> main_RETURN;
    second_deadlock_BASIC_BLOCK_1 -> second_deadlock_BLOCK_1_STATEMENT_0;
    second_deadlock_BASIC_BLOCK_2 -> second_deadlock_BLOCK_2_STATEMENT_0;
    second_deadlock_BASIC_BLOCK_3 -> second_deadlock_DROP_3;
    second_deadlock_BASIC_BLOCK_3 -> second_deadlock_DROP_UNWIND_3;
    second_deadlock_BASIC_BLOCK_4 -> second_deadlock_DROP_4;
    second_deadlock_BASIC_BLOCK_5 -> second_deadlock_RETURN;
    second_deadlock_BASIC_BLOCK_6 -> second_deadlock_DROP_6;
    second_deadlock_BASIC_BLOCK_7 -> second_deadlock_UNWIND_7;
    second_deadlock_BASIC_BLOCK_END_PLACE_1 -> std_sync_Mutex_T_lock_2;
    second_deadlock_BASIC_BLOCK_END_PLACE_2 -> std_sync_Mutex_T_lock_3;
    first_deadlock_BLOCK_1_STATEMENT_0 -> first_deadlock_BASIC_BLOCK_END_PLACE_1;
    first_deadlock_BLOCK_2_STATEMENT_0 -> first_deadlock_BASIC_BLOCK_END_PLACE_2;
    first_deadlock_DROP_3 -> first_deadlock_BASIC_BLOCK_4;
    first_deadlock_DROP_4 -> first_deadlock_BASIC_BLOCK_5;
    first_deadlock_DROP_6 -> first_deadlock_BASIC_BLOCK_7;
    first_deadlock_DROP_UNWIND_3 -> first_deadlock_BASIC_BLOCK_6;
    first_deadlock_RETURN -> main_BASIC_BLOCK_1;
    first_deadlock_UNWIND_7 -> PROGRAM_PANIC;
    main_RETURN -> PROGRAM_END;
    second_deadlock_BLOCK_1_STATEMENT_0 -> second_deadlock_BASIC_BLOCK_END_PLACE_1;
    second_deadlock_BLOCK_2_STATEMENT_0 -> second_deadlock_BASIC_BLOCK_END_PLACE_2;
    second_deadlock_DROP_3 -> second_deadlock_BASIC_BLOCK_4;
    second_deadlock_DROP_4 -> second_deadlock_BASIC_BLOCK_5;
    second_deadlock_DROP_6 -> second_deadlock_BASIC_BLOCK_7;
    second_deadlock_DROP_UNWIND_3 -> second_deadlock_BASIC_BLOCK_6;
    second_deadlock_RETURN -> main_BASIC_BLOCK_2;
    second_deadlock_UNWIND_7 -> PROGRAM_PANIC;
    std_sync_Mutex_T_lock_0 -> first_deadlock_BASIC_BLOCK_2;
    std_sync_Mutex_T_lock_1 -> first_deadlock_BASIC_BLOCK_3;
    std_sync_Mutex_T_lock_1 -> first_deadlock_BASIC_BLOCK_6;
    std_sync_Mutex_T_lock_2 -> second_deadlock_BASIC_BLOCK_2;
    std_sync_Mutex_T_lock_3 -> second_deadlock_BASIC_BLOCK_3;
    std_sync_Mutex_T_lock_3 -> second_deadlock_BASIC_BLOCK_6;
    std_sync_Mutex_T_new_0 -> first_deadlock_BASIC_BLOCK_1;
    std_sync_Mutex_T_new_1 -> second_deadlock_BASIC_BLOCK_1;
}
"#;

const TWO_MUTEXES_TWO_FUNCTIONS_LOLA_OUTPUT: &str = r#"PLACE
    MUTEX_0,
    MUTEX_1,
    PROGRAM_END,
    PROGRAM_PANIC,
    PROGRAM_START,
    first_deadlock_BASIC_BLOCK_1,
    first_deadlock_BASIC_BLOCK_2,
    first_deadlock_BASIC_BLOCK_3,
    first_deadlock_BASIC_BLOCK_4,
    first_deadlock_BASIC_BLOCK_5,
    first_deadlock_BASIC_BLOCK_6,
    first_deadlock_BASIC_BLOCK_7,
    first_deadlock_BASIC_BLOCK_END_PLACE_1,
    first_deadlock_BASIC_BLOCK_END_PLACE_2,
    main_BASIC_BLOCK_1,
    main_BASIC_BLOCK_2,
    second_deadlock_BASIC_BLOCK_1,
    second_deadlock_BASIC_BLOCK_2,
    second_deadlock_BASIC_BLOCK_3,
    second_deadlock_BASIC_BLOCK_4,
    second_deadlock_BASIC_BLOCK_5,
    second_deadlock_BASIC_BLOCK_6,
    second_deadlock_BASIC_BLOCK_7,
    second_deadlock_BASIC_BLOCK_END_PLACE_1,
    second_deadlock_BASIC_BLOCK_END_PLACE_2;

MARKING
    MUTEX_0 : 1,
    MUTEX_1 : 1,
    PROGRAM_START : 1,
TRANSITION first_deadlock_BLOCK_1_STATEMENT_0
  CONSUME
    first_deadlock_BASIC_BLOCK_1 : 1;
  PRODUCE
    first_deadlock_BASIC_BLOCK_END_PLACE_1 : 1;
TRANSITION first_deadlock_BLOCK_2_STATEMENT_0
  CONSUME
    first_deadlock_BASIC_BLOCK_2 : 1;
  PRODUCE
    first_deadlock_BASIC_BLOCK_END_PLACE_2 : 1;
TRANSITION first_deadlock_DROP_3
  CONSUME
    first_deadlock_BASIC_BLOCK_3 : 1;
  PRODUCE
    first_deadlock_BASIC_BLOCK_4 : 1;
TRANSITION first_deadlock_DROP_4
  CONSUME
    first_deadlock_BASIC_BLOCK_4 : 1;
  PRODUCE
    first_deadlock_BASIC_BLOCK_5 : 1;
TRANSITION first_deadlock_DROP_6
  CONSUME
    first_deadlock_BASIC_BLOCK_6 : 1;
  PRODUCE
    first_deadlock_BASIC_BLOCK_7 : 1;
TRANSITION first_deadlock_DROP_UNWIND_3
  CONSUME
    first_deadlock_BASIC_BLOCK_3 : 1;
  PRODUCE
    first_deadlock_BASIC_BLOCK_6 : 1;
TRANSITION first_deadlock_RETURN
  CONSUME
    first_deadlock_BASIC_BLOCK_5 : 1;
  PRODUCE
    main_BASIC_BLOCK_1 : 1;
TRANSITION first_deadlock_UNWIND_7
  CONSUME
    first_deadlock_BASIC_BLOCK_7 : 1;
  PRODUCE
    PROGRAM_PANIC : 1;
TRANSITION main_RETURN
  CONSUME
    main_BASIC_BLOCK_2 : 1;
  PRODUCE
    PROGRAM_END : 1;
TRANSITION second_deadlock_BLOCK_1_STATEMENT_0
  CONSUME
    second_deadlock_BASIC_BLOCK_1 : 1;
  PRODUCE
    second_deadlock_BASIC_BLOCK_END_PLACE_1 : 1;
TRANSITION second_deadlock_BLOCK_2_STATEMENT_0
  CONSUME
    second_deadlock_BASIC_BLOCK_2 : 1;
  PRODUCE
    second_deadlock_BASIC_BLOCK_END_PLACE_2 : 1;
TRANSITION second_deadlock_DROP_3
  CONSUME
    second_deadlock_BASIC_BLOCK_3 : 1;
  PRODUCE
    second_deadlock_BASIC_BLOCK_4 : 1;
TRANSITION second_deadlock_DROP_4
  CONSUME
    second_deadlock_BASIC_BLOCK_4 : 1;
  PRODUCE
    second_deadlock_BASIC_BLOCK_5 : 1;
TRANSITION second_deadlock_DROP_6
  CONSUME
    second_deadlock_BASIC_BLOCK_6 : 1;
  PRODUCE
    second_deadlock_BASIC_BLOCK_7 : 1;
TRANSITION second_deadlock_DROP_UNWIND_3
  CONSUME
    second_deadlock_BASIC_BLOCK_3 : 1;
  PRODUCE
    second_deadlock_BASIC_BLOCK_6 : 1;
TRANSITION second_deadlock_RETURN
  CONSUME
    second_deadlock_BASIC_BLOCK_5 : 1;
  PRODUCE
    main_BASIC_BLOCK_2 : 1;
TRANSITION second_deadlock_UNWIND_7
  CONSUME
    second_deadlock_BASIC_BLOCK_7 : 1;
  PRODUCE
    PROGRAM_PANIC : 1;
TRANSITION std_sync_Mutex_T_lock_0
  CONSUME
    MUTEX_0 : 1,
    first_deadlock_BASIC_BLOCK_END_PLACE_1 : 1;
  PRODUCE
    first_deadlock_BASIC_BLOCK_2 : 1;
TRANSITION std_sync_Mutex_T_lock_1
  CONSUME
    MUTEX_0 : 1,
    first_deadlock_BASIC_BLOCK_END_PLACE_2 : 1;
  PRODUCE
    first_deadlock_BASIC_BLOCK_3 : 1,
    first_deadlock_BASIC_BLOCK_6 : 1;
TRANSITION std_sync_Mutex_T_lock_2
  CONSUME
    MUTEX_1 : 1,
    second_deadlock_BASIC_BLOCK_END_PLACE_1 : 1;
  PRODUCE
    second_deadlock_BASIC_BLOCK_2 : 1;
TRANSITION std_sync_Mutex_T_lock_3
  CONSUME
    MUTEX_1 : 1,
    second_deadlock_BASIC_BLOCK_END_PLACE_2 : 1;
  PRODUCE
    second_deadlock_BASIC_BLOCK_3 : 1,
    second_deadlock_BASIC_BLOCK_6 : 1;
TRANSITION std_sync_Mutex_T_new_0
  CONSUME
    PROGRAM_START : 1;
  PRODUCE
    first_deadlock_BASIC_BLOCK_1 : 1;
TRANSITION std_sync_Mutex_T_new_1
  CONSUME
    main_BASIC_BLOCK_1 : 1;
  PRODUCE
    second_deadlock_BASIC_BLOCK_1 : 1;
"#;

const TWO_MUTEXES_TWO_FUNCTIONS_PNML_OUTPUT: &str = r#"<?xml version="1.0" encoding="utf-8"?>
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
      <place id="MUTEX_1">
        <name>
          <text>MUTEX_1</text>
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
      <place id="first_deadlock_BASIC_BLOCK_1">
        <name>
          <text>first_deadlock_BASIC_BLOCK_1</text>
        </name>
      </place>
      <place id="first_deadlock_BASIC_BLOCK_2">
        <name>
          <text>first_deadlock_BASIC_BLOCK_2</text>
        </name>
      </place>
      <place id="first_deadlock_BASIC_BLOCK_3">
        <name>
          <text>first_deadlock_BASIC_BLOCK_3</text>
        </name>
      </place>
      <place id="first_deadlock_BASIC_BLOCK_4">
        <name>
          <text>first_deadlock_BASIC_BLOCK_4</text>
        </name>
      </place>
      <place id="first_deadlock_BASIC_BLOCK_5">
        <name>
          <text>first_deadlock_BASIC_BLOCK_5</text>
        </name>
      </place>
      <place id="first_deadlock_BASIC_BLOCK_6">
        <name>
          <text>first_deadlock_BASIC_BLOCK_6</text>
        </name>
      </place>
      <place id="first_deadlock_BASIC_BLOCK_7">
        <name>
          <text>first_deadlock_BASIC_BLOCK_7</text>
        </name>
      </place>
      <place id="first_deadlock_BASIC_BLOCK_END_PLACE_1">
        <name>
          <text>first_deadlock_BASIC_BLOCK_END_PLACE_1</text>
        </name>
      </place>
      <place id="first_deadlock_BASIC_BLOCK_END_PLACE_2">
        <name>
          <text>first_deadlock_BASIC_BLOCK_END_PLACE_2</text>
        </name>
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
      <place id="second_deadlock_BASIC_BLOCK_1">
        <name>
          <text>second_deadlock_BASIC_BLOCK_1</text>
        </name>
      </place>
      <place id="second_deadlock_BASIC_BLOCK_2">
        <name>
          <text>second_deadlock_BASIC_BLOCK_2</text>
        </name>
      </place>
      <place id="second_deadlock_BASIC_BLOCK_3">
        <name>
          <text>second_deadlock_BASIC_BLOCK_3</text>
        </name>
      </place>
      <place id="second_deadlock_BASIC_BLOCK_4">
        <name>
          <text>second_deadlock_BASIC_BLOCK_4</text>
        </name>
      </place>
      <place id="second_deadlock_BASIC_BLOCK_5">
        <name>
          <text>second_deadlock_BASIC_BLOCK_5</text>
        </name>
      </place>
      <place id="second_deadlock_BASIC_BLOCK_6">
        <name>
          <text>second_deadlock_BASIC_BLOCK_6</text>
        </name>
      </place>
      <place id="second_deadlock_BASIC_BLOCK_7">
        <name>
          <text>second_deadlock_BASIC_BLOCK_7</text>
        </name>
      </place>
      <place id="second_deadlock_BASIC_BLOCK_END_PLACE_1">
        <name>
          <text>second_deadlock_BASIC_BLOCK_END_PLACE_1</text>
        </name>
      </place>
      <place id="second_deadlock_BASIC_BLOCK_END_PLACE_2">
        <name>
          <text>second_deadlock_BASIC_BLOCK_END_PLACE_2</text>
        </name>
      </place>
      <transition id="first_deadlock_BLOCK_1_STATEMENT_0">
        <name>
          <text>first_deadlock_BLOCK_1_STATEMENT_0</text>
        </name>
      </transition>
      <transition id="first_deadlock_BLOCK_2_STATEMENT_0">
        <name>
          <text>first_deadlock_BLOCK_2_STATEMENT_0</text>
        </name>
      </transition>
      <transition id="first_deadlock_DROP_3">
        <name>
          <text>first_deadlock_DROP_3</text>
        </name>
      </transition>
      <transition id="first_deadlock_DROP_4">
        <name>
          <text>first_deadlock_DROP_4</text>
        </name>
      </transition>
      <transition id="first_deadlock_DROP_6">
        <name>
          <text>first_deadlock_DROP_6</text>
        </name>
      </transition>
      <transition id="first_deadlock_DROP_UNWIND_3">
        <name>
          <text>first_deadlock_DROP_UNWIND_3</text>
        </name>
      </transition>
      <transition id="first_deadlock_RETURN">
        <name>
          <text>first_deadlock_RETURN</text>
        </name>
      </transition>
      <transition id="first_deadlock_UNWIND_7">
        <name>
          <text>first_deadlock_UNWIND_7</text>
        </name>
      </transition>
      <transition id="main_RETURN">
        <name>
          <text>main_RETURN</text>
        </name>
      </transition>
      <transition id="second_deadlock_BLOCK_1_STATEMENT_0">
        <name>
          <text>second_deadlock_BLOCK_1_STATEMENT_0</text>
        </name>
      </transition>
      <transition id="second_deadlock_BLOCK_2_STATEMENT_0">
        <name>
          <text>second_deadlock_BLOCK_2_STATEMENT_0</text>
        </name>
      </transition>
      <transition id="second_deadlock_DROP_3">
        <name>
          <text>second_deadlock_DROP_3</text>
        </name>
      </transition>
      <transition id="second_deadlock_DROP_4">
        <name>
          <text>second_deadlock_DROP_4</text>
        </name>
      </transition>
      <transition id="second_deadlock_DROP_6">
        <name>
          <text>second_deadlock_DROP_6</text>
        </name>
      </transition>
      <transition id="second_deadlock_DROP_UNWIND_3">
        <name>
          <text>second_deadlock_DROP_UNWIND_3</text>
        </name>
      </transition>
      <transition id="second_deadlock_RETURN">
        <name>
          <text>second_deadlock_RETURN</text>
        </name>
      </transition>
      <transition id="second_deadlock_UNWIND_7">
        <name>
          <text>second_deadlock_UNWIND_7</text>
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
      <transition id="std_sync_Mutex_T_lock_2">
        <name>
          <text>std_sync_Mutex_T_lock_2</text>
        </name>
      </transition>
      <transition id="std_sync_Mutex_T_lock_3">
        <name>
          <text>std_sync_Mutex_T_lock_3</text>
        </name>
      </transition>
      <transition id="std_sync_Mutex_T_new_0">
        <name>
          <text>std_sync_Mutex_T_new_0</text>
        </name>
      </transition>
      <transition id="std_sync_Mutex_T_new_1">
        <name>
          <text>std_sync_Mutex_T_new_1</text>
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
      <arc source="MUTEX_1" target="std_sync_Mutex_T_lock_2" id="(MUTEX_1, std_sync_Mutex_T_lock_2)">
        <name>
          <text>(MUTEX_1, std_sync_Mutex_T_lock_2)</text>
        </name>
        <inscription>
          <text>1</text>
        </inscription>
      </arc>
      <arc source="MUTEX_1" target="std_sync_Mutex_T_lock_3" id="(MUTEX_1, std_sync_Mutex_T_lock_3)">
        <name>
          <text>(MUTEX_1, std_sync_Mutex_T_lock_3)</text>
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
      <arc source="first_deadlock_BASIC_BLOCK_1" target="first_deadlock_BLOCK_1_STATEMENT_0" id="(first_deadlock_BASIC_BLOCK_1, first_deadlock_BLOCK_1_STATEMENT_0)">
        <name>
          <text>(first_deadlock_BASIC_BLOCK_1, first_deadlock_BLOCK_1_STATEMENT_0)</text>
        </name>
        <inscription>
          <text>1</text>
        </inscription>
      </arc>
      <arc source="first_deadlock_BASIC_BLOCK_2" target="first_deadlock_BLOCK_2_STATEMENT_0" id="(first_deadlock_BASIC_BLOCK_2, first_deadlock_BLOCK_2_STATEMENT_0)">
        <name>
          <text>(first_deadlock_BASIC_BLOCK_2, first_deadlock_BLOCK_2_STATEMENT_0)</text>
        </name>
        <inscription>
          <text>1</text>
        </inscription>
      </arc>
      <arc source="first_deadlock_BASIC_BLOCK_3" target="first_deadlock_DROP_3" id="(first_deadlock_BASIC_BLOCK_3, first_deadlock_DROP_3)">
        <name>
          <text>(first_deadlock_BASIC_BLOCK_3, first_deadlock_DROP_3)</text>
        </name>
        <inscription>
          <text>1</text>
        </inscription>
      </arc>
      <arc source="first_deadlock_BASIC_BLOCK_3" target="first_deadlock_DROP_UNWIND_3" id="(first_deadlock_BASIC_BLOCK_3, first_deadlock_DROP_UNWIND_3)">
        <name>
          <text>(first_deadlock_BASIC_BLOCK_3, first_deadlock_DROP_UNWIND_3)</text>
        </name>
        <inscription>
          <text>1</text>
        </inscription>
      </arc>
      <arc source="first_deadlock_BASIC_BLOCK_4" target="first_deadlock_DROP_4" id="(first_deadlock_BASIC_BLOCK_4, first_deadlock_DROP_4)">
        <name>
          <text>(first_deadlock_BASIC_BLOCK_4, first_deadlock_DROP_4)</text>
        </name>
        <inscription>
          <text>1</text>
        </inscription>
      </arc>
      <arc source="first_deadlock_BASIC_BLOCK_5" target="first_deadlock_RETURN" id="(first_deadlock_BASIC_BLOCK_5, first_deadlock_RETURN)">
        <name>
          <text>(first_deadlock_BASIC_BLOCK_5, first_deadlock_RETURN)</text>
        </name>
        <inscription>
          <text>1</text>
        </inscription>
      </arc>
      <arc source="first_deadlock_BASIC_BLOCK_6" target="first_deadlock_DROP_6" id="(first_deadlock_BASIC_BLOCK_6, first_deadlock_DROP_6)">
        <name>
          <text>(first_deadlock_BASIC_BLOCK_6, first_deadlock_DROP_6)</text>
        </name>
        <inscription>
          <text>1</text>
        </inscription>
      </arc>
      <arc source="first_deadlock_BASIC_BLOCK_7" target="first_deadlock_UNWIND_7" id="(first_deadlock_BASIC_BLOCK_7, first_deadlock_UNWIND_7)">
        <name>
          <text>(first_deadlock_BASIC_BLOCK_7, first_deadlock_UNWIND_7)</text>
        </name>
        <inscription>
          <text>1</text>
        </inscription>
      </arc>
      <arc source="first_deadlock_BASIC_BLOCK_END_PLACE_1" target="std_sync_Mutex_T_lock_0" id="(first_deadlock_BASIC_BLOCK_END_PLACE_1, std_sync_Mutex_T_lock_0)">
        <name>
          <text>(first_deadlock_BASIC_BLOCK_END_PLACE_1, std_sync_Mutex_T_lock_0)</text>
        </name>
        <inscription>
          <text>1</text>
        </inscription>
      </arc>
      <arc source="first_deadlock_BASIC_BLOCK_END_PLACE_2" target="std_sync_Mutex_T_lock_1" id="(first_deadlock_BASIC_BLOCK_END_PLACE_2, std_sync_Mutex_T_lock_1)">
        <name>
          <text>(first_deadlock_BASIC_BLOCK_END_PLACE_2, std_sync_Mutex_T_lock_1)</text>
        </name>
        <inscription>
          <text>1</text>
        </inscription>
      </arc>
      <arc source="main_BASIC_BLOCK_1" target="std_sync_Mutex_T_new_1" id="(main_BASIC_BLOCK_1, std_sync_Mutex_T_new_1)">
        <name>
          <text>(main_BASIC_BLOCK_1, std_sync_Mutex_T_new_1)</text>
        </name>
        <inscription>
          <text>1</text>
        </inscription>
      </arc>
      <arc source="main_BASIC_BLOCK_2" target="main_RETURN" id="(main_BASIC_BLOCK_2, main_RETURN)">
        <name>
          <text>(main_BASIC_BLOCK_2, main_RETURN)</text>
        </name>
        <inscription>
          <text>1</text>
        </inscription>
      </arc>
      <arc source="second_deadlock_BASIC_BLOCK_1" target="second_deadlock_BLOCK_1_STATEMENT_0" id="(second_deadlock_BASIC_BLOCK_1, second_deadlock_BLOCK_1_STATEMENT_0)">
        <name>
          <text>(second_deadlock_BASIC_BLOCK_1, second_deadlock_BLOCK_1_STATEMENT_0)</text>
        </name>
        <inscription>
          <text>1</text>
        </inscription>
      </arc>
      <arc source="second_deadlock_BASIC_BLOCK_2" target="second_deadlock_BLOCK_2_STATEMENT_0" id="(second_deadlock_BASIC_BLOCK_2, second_deadlock_BLOCK_2_STATEMENT_0)">
        <name>
          <text>(second_deadlock_BASIC_BLOCK_2, second_deadlock_BLOCK_2_STATEMENT_0)</text>
        </name>
        <inscription>
          <text>1</text>
        </inscription>
      </arc>
      <arc source="second_deadlock_BASIC_BLOCK_3" target="second_deadlock_DROP_3" id="(second_deadlock_BASIC_BLOCK_3, second_deadlock_DROP_3)">
        <name>
          <text>(second_deadlock_BASIC_BLOCK_3, second_deadlock_DROP_3)</text>
        </name>
        <inscription>
          <text>1</text>
        </inscription>
      </arc>
      <arc source="second_deadlock_BASIC_BLOCK_3" target="second_deadlock_DROP_UNWIND_3" id="(second_deadlock_BASIC_BLOCK_3, second_deadlock_DROP_UNWIND_3)">
        <name>
          <text>(second_deadlock_BASIC_BLOCK_3, second_deadlock_DROP_UNWIND_3)</text>
        </name>
        <inscription>
          <text>1</text>
        </inscription>
      </arc>
      <arc source="second_deadlock_BASIC_BLOCK_4" target="second_deadlock_DROP_4" id="(second_deadlock_BASIC_BLOCK_4, second_deadlock_DROP_4)">
        <name>
          <text>(second_deadlock_BASIC_BLOCK_4, second_deadlock_DROP_4)</text>
        </name>
        <inscription>
          <text>1</text>
        </inscription>
      </arc>
      <arc source="second_deadlock_BASIC_BLOCK_5" target="second_deadlock_RETURN" id="(second_deadlock_BASIC_BLOCK_5, second_deadlock_RETURN)">
        <name>
          <text>(second_deadlock_BASIC_BLOCK_5, second_deadlock_RETURN)</text>
        </name>
        <inscription>
          <text>1</text>
        </inscription>
      </arc>
      <arc source="second_deadlock_BASIC_BLOCK_6" target="second_deadlock_DROP_6" id="(second_deadlock_BASIC_BLOCK_6, second_deadlock_DROP_6)">
        <name>
          <text>(second_deadlock_BASIC_BLOCK_6, second_deadlock_DROP_6)</text>
        </name>
        <inscription>
          <text>1</text>
        </inscription>
      </arc>
      <arc source="second_deadlock_BASIC_BLOCK_7" target="second_deadlock_UNWIND_7" id="(second_deadlock_BASIC_BLOCK_7, second_deadlock_UNWIND_7)">
        <name>
          <text>(second_deadlock_BASIC_BLOCK_7, second_deadlock_UNWIND_7)</text>
        </name>
        <inscription>
          <text>1</text>
        </inscription>
      </arc>
      <arc source="second_deadlock_BASIC_BLOCK_END_PLACE_1" target="std_sync_Mutex_T_lock_2" id="(second_deadlock_BASIC_BLOCK_END_PLACE_1, std_sync_Mutex_T_lock_2)">
        <name>
          <text>(second_deadlock_BASIC_BLOCK_END_PLACE_1, std_sync_Mutex_T_lock_2)</text>
        </name>
        <inscription>
          <text>1</text>
        </inscription>
      </arc>
      <arc source="second_deadlock_BASIC_BLOCK_END_PLACE_2" target="std_sync_Mutex_T_lock_3" id="(second_deadlock_BASIC_BLOCK_END_PLACE_2, std_sync_Mutex_T_lock_3)">
        <name>
          <text>(second_deadlock_BASIC_BLOCK_END_PLACE_2, std_sync_Mutex_T_lock_3)</text>
        </name>
        <inscription>
          <text>1</text>
        </inscription>
      </arc>
      <arc source="first_deadlock_BLOCK_1_STATEMENT_0" target="first_deadlock_BASIC_BLOCK_END_PLACE_1" id="(first_deadlock_BLOCK_1_STATEMENT_0, first_deadlock_BASIC_BLOCK_END_PLACE_1)">
        <name>
          <text>(first_deadlock_BLOCK_1_STATEMENT_0, first_deadlock_BASIC_BLOCK_END_PLACE_1)</text>
        </name>
        <inscription>
          <text>1</text>
        </inscription>
      </arc>
      <arc source="first_deadlock_BLOCK_2_STATEMENT_0" target="first_deadlock_BASIC_BLOCK_END_PLACE_2" id="(first_deadlock_BLOCK_2_STATEMENT_0, first_deadlock_BASIC_BLOCK_END_PLACE_2)">
        <name>
          <text>(first_deadlock_BLOCK_2_STATEMENT_0, first_deadlock_BASIC_BLOCK_END_PLACE_2)</text>
        </name>
        <inscription>
          <text>1</text>
        </inscription>
      </arc>
      <arc source="first_deadlock_DROP_3" target="first_deadlock_BASIC_BLOCK_4" id="(first_deadlock_DROP_3, first_deadlock_BASIC_BLOCK_4)">
        <name>
          <text>(first_deadlock_DROP_3, first_deadlock_BASIC_BLOCK_4)</text>
        </name>
        <inscription>
          <text>1</text>
        </inscription>
      </arc>
      <arc source="first_deadlock_DROP_4" target="first_deadlock_BASIC_BLOCK_5" id="(first_deadlock_DROP_4, first_deadlock_BASIC_BLOCK_5)">
        <name>
          <text>(first_deadlock_DROP_4, first_deadlock_BASIC_BLOCK_5)</text>
        </name>
        <inscription>
          <text>1</text>
        </inscription>
      </arc>
      <arc source="first_deadlock_DROP_6" target="first_deadlock_BASIC_BLOCK_7" id="(first_deadlock_DROP_6, first_deadlock_BASIC_BLOCK_7)">
        <name>
          <text>(first_deadlock_DROP_6, first_deadlock_BASIC_BLOCK_7)</text>
        </name>
        <inscription>
          <text>1</text>
        </inscription>
      </arc>
      <arc source="first_deadlock_DROP_UNWIND_3" target="first_deadlock_BASIC_BLOCK_6" id="(first_deadlock_DROP_UNWIND_3, first_deadlock_BASIC_BLOCK_6)">
        <name>
          <text>(first_deadlock_DROP_UNWIND_3, first_deadlock_BASIC_BLOCK_6)</text>
        </name>
        <inscription>
          <text>1</text>
        </inscription>
      </arc>
      <arc source="first_deadlock_RETURN" target="main_BASIC_BLOCK_1" id="(first_deadlock_RETURN, main_BASIC_BLOCK_1)">
        <name>
          <text>(first_deadlock_RETURN, main_BASIC_BLOCK_1)</text>
        </name>
        <inscription>
          <text>1</text>
        </inscription>
      </arc>
      <arc source="first_deadlock_UNWIND_7" target="PROGRAM_PANIC" id="(first_deadlock_UNWIND_7, PROGRAM_PANIC)">
        <name>
          <text>(first_deadlock_UNWIND_7, PROGRAM_PANIC)</text>
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
      <arc source="second_deadlock_BLOCK_1_STATEMENT_0" target="second_deadlock_BASIC_BLOCK_END_PLACE_1" id="(second_deadlock_BLOCK_1_STATEMENT_0, second_deadlock_BASIC_BLOCK_END_PLACE_1)">
        <name>
          <text>(second_deadlock_BLOCK_1_STATEMENT_0, second_deadlock_BASIC_BLOCK_END_PLACE_1)</text>
        </name>
        <inscription>
          <text>1</text>
        </inscription>
      </arc>
      <arc source="second_deadlock_BLOCK_2_STATEMENT_0" target="second_deadlock_BASIC_BLOCK_END_PLACE_2" id="(second_deadlock_BLOCK_2_STATEMENT_0, second_deadlock_BASIC_BLOCK_END_PLACE_2)">
        <name>
          <text>(second_deadlock_BLOCK_2_STATEMENT_0, second_deadlock_BASIC_BLOCK_END_PLACE_2)</text>
        </name>
        <inscription>
          <text>1</text>
        </inscription>
      </arc>
      <arc source="second_deadlock_DROP_3" target="second_deadlock_BASIC_BLOCK_4" id="(second_deadlock_DROP_3, second_deadlock_BASIC_BLOCK_4)">
        <name>
          <text>(second_deadlock_DROP_3, second_deadlock_BASIC_BLOCK_4)</text>
        </name>
        <inscription>
          <text>1</text>
        </inscription>
      </arc>
      <arc source="second_deadlock_DROP_4" target="second_deadlock_BASIC_BLOCK_5" id="(second_deadlock_DROP_4, second_deadlock_BASIC_BLOCK_5)">
        <name>
          <text>(second_deadlock_DROP_4, second_deadlock_BASIC_BLOCK_5)</text>
        </name>
        <inscription>
          <text>1</text>
        </inscription>
      </arc>
      <arc source="second_deadlock_DROP_6" target="second_deadlock_BASIC_BLOCK_7" id="(second_deadlock_DROP_6, second_deadlock_BASIC_BLOCK_7)">
        <name>
          <text>(second_deadlock_DROP_6, second_deadlock_BASIC_BLOCK_7)</text>
        </name>
        <inscription>
          <text>1</text>
        </inscription>
      </arc>
      <arc source="second_deadlock_DROP_UNWIND_3" target="second_deadlock_BASIC_BLOCK_6" id="(second_deadlock_DROP_UNWIND_3, second_deadlock_BASIC_BLOCK_6)">
        <name>
          <text>(second_deadlock_DROP_UNWIND_3, second_deadlock_BASIC_BLOCK_6)</text>
        </name>
        <inscription>
          <text>1</text>
        </inscription>
      </arc>
      <arc source="second_deadlock_RETURN" target="main_BASIC_BLOCK_2" id="(second_deadlock_RETURN, main_BASIC_BLOCK_2)">
        <name>
          <text>(second_deadlock_RETURN, main_BASIC_BLOCK_2)</text>
        </name>
        <inscription>
          <text>1</text>
        </inscription>
      </arc>
      <arc source="second_deadlock_UNWIND_7" target="PROGRAM_PANIC" id="(second_deadlock_UNWIND_7, PROGRAM_PANIC)">
        <name>
          <text>(second_deadlock_UNWIND_7, PROGRAM_PANIC)</text>
        </name>
        <inscription>
          <text>1</text>
        </inscription>
      </arc>
      <arc source="std_sync_Mutex_T_lock_0" target="first_deadlock_BASIC_BLOCK_2" id="(std_sync_Mutex_T_lock_0, first_deadlock_BASIC_BLOCK_2)">
        <name>
          <text>(std_sync_Mutex_T_lock_0, first_deadlock_BASIC_BLOCK_2)</text>
        </name>
        <inscription>
          <text>1</text>
        </inscription>
      </arc>
      <arc source="std_sync_Mutex_T_lock_1" target="first_deadlock_BASIC_BLOCK_3" id="(std_sync_Mutex_T_lock_1, first_deadlock_BASIC_BLOCK_3)">
        <name>
          <text>(std_sync_Mutex_T_lock_1, first_deadlock_BASIC_BLOCK_3)</text>
        </name>
        <inscription>
          <text>1</text>
        </inscription>
      </arc>
      <arc source="std_sync_Mutex_T_lock_1" target="first_deadlock_BASIC_BLOCK_6" id="(std_sync_Mutex_T_lock_1, first_deadlock_BASIC_BLOCK_6)">
        <name>
          <text>(std_sync_Mutex_T_lock_1, first_deadlock_BASIC_BLOCK_6)</text>
        </name>
        <inscription>
          <text>1</text>
        </inscription>
      </arc>
      <arc source="std_sync_Mutex_T_lock_2" target="second_deadlock_BASIC_BLOCK_2" id="(std_sync_Mutex_T_lock_2, second_deadlock_BASIC_BLOCK_2)">
        <name>
          <text>(std_sync_Mutex_T_lock_2, second_deadlock_BASIC_BLOCK_2)</text>
        </name>
        <inscription>
          <text>1</text>
        </inscription>
      </arc>
      <arc source="std_sync_Mutex_T_lock_3" target="second_deadlock_BASIC_BLOCK_3" id="(std_sync_Mutex_T_lock_3, second_deadlock_BASIC_BLOCK_3)">
        <name>
          <text>(std_sync_Mutex_T_lock_3, second_deadlock_BASIC_BLOCK_3)</text>
        </name>
        <inscription>
          <text>1</text>
        </inscription>
      </arc>
      <arc source="std_sync_Mutex_T_lock_3" target="second_deadlock_BASIC_BLOCK_6" id="(std_sync_Mutex_T_lock_3, second_deadlock_BASIC_BLOCK_6)">
        <name>
          <text>(std_sync_Mutex_T_lock_3, second_deadlock_BASIC_BLOCK_6)</text>
        </name>
        <inscription>
          <text>1</text>
        </inscription>
      </arc>
      <arc source="std_sync_Mutex_T_new_0" target="first_deadlock_BASIC_BLOCK_1" id="(std_sync_Mutex_T_new_0, first_deadlock_BASIC_BLOCK_1)">
        <name>
          <text>(std_sync_Mutex_T_new_0, first_deadlock_BASIC_BLOCK_1)</text>
        </name>
        <inscription>
          <text>1</text>
        </inscription>
      </arc>
      <arc source="std_sync_Mutex_T_new_1" target="second_deadlock_BASIC_BLOCK_1" id="(std_sync_Mutex_T_new_1, second_deadlock_BASIC_BLOCK_1)">
        <name>
          <text>(std_sync_Mutex_T_new_1, second_deadlock_BASIC_BLOCK_1)</text>
        </name>
        <inscription>
          <text>1</text>
        </inscription>
      </arc>
    </page>
  </net>
</pnml>"#;

#[test]
fn two_mutexes_two_functions_generates_correct_dot_output_file() {
    assert_output_file(
        "./tests/sample_programs/two_mutexes_two_functions.rs",
        "dot",
        "./net.dot",
        TWO_MUTEXES_TWO_FUNCTIONS_DOT_OUTPUT,
    );
}

#[test]
fn two_mutexes_two_functions_generates_correct_lola_output_file() {
    assert_output_file(
        "./tests/sample_programs/two_mutexes_two_functions.rs",
        "lola",
        "./net.lola",
        TWO_MUTEXES_TWO_FUNCTIONS_LOLA_OUTPUT,
    );
}

#[test]
fn two_mutexes_two_functions_generates_correct_pnml_output_file() {
    assert_output_file(
        "./tests/sample_programs/two_mutexes_two_functions.rs",
        "pnml",
        "./net.pnml",
        TWO_MUTEXES_TWO_FUNCTIONS_PNML_OUTPUT,
    );
}
