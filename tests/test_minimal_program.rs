mod common;
use common::assert_output_file;

const MINIMAL_PROGRAM_DOT_OUTPUT: &str = "\
digraph petrinet {
    BASIC_BLOCK_END_PLACE [shape=\"circle\" xlabel=\"BASIC_BLOCK_END_PLACE\" label=\"\"];
    PROGRAM_END [shape=\"circle\" xlabel=\"PROGRAM_END\" label=\"\"];
    PROGRAM_PANIC [shape=\"circle\" xlabel=\"PROGRAM_PANIC\" label=\"\"];
    PROGRAM_START [shape=\"circle\" xlabel=\"PROGRAM_START\" label=\"•\"];
    BASIC_BLOCK_EMPTY [shape=\"box\" xlabel=\"BASIC_BLOCK_EMPTY\" label=\"\"];
    RETURN_main [shape=\"box\" xlabel=\"RETURN_main\" label=\"\"];
    BASIC_BLOCK_END_PLACE -> RETURN_main;
    PROGRAM_START -> BASIC_BLOCK_EMPTY;
    BASIC_BLOCK_EMPTY -> BASIC_BLOCK_END_PLACE;
    RETURN_main -> PROGRAM_END;
}
";

const MINIMAL_PROGRAM_LOLA_OUTPUT: &str = "\
PLACE
    BASIC_BLOCK_END_PLACE,
    PROGRAM_END,
    PROGRAM_PANIC,
    PROGRAM_START;

MARKING
    PROGRAM_START : 1;

TRANSITION BASIC_BLOCK_EMPTY
  CONSUME
    PROGRAM_START : 1;
  PRODUCE
    BASIC_BLOCK_END_PLACE : 1;
TRANSITION RETURN_main
  CONSUME
    BASIC_BLOCK_END_PLACE : 1;
  PRODUCE
    PROGRAM_END : 1;
";

const MINIMAL_PROGRAM_PNML_OUTPUT: &str = "\
<?xml version=\"1.0\" encoding=\"utf-8\"?>
<pnml xmlns=\"http://www.pnml.org/version-2009/grammar/pnml\">
  <net id=\"net0\" type=\"http://www.pnml.org/version-2009/grammar/ptnet\">
    <page id=\"page0\">
      <place id=\"BASIC_BLOCK_END_PLACE\">
        <name>
          <text>BASIC_BLOCK_END_PLACE</text>
        </name>
      </place>
      <place id=\"PROGRAM_END\">
        <name>
          <text>PROGRAM_END</text>
        </name>
      </place>
      <place id=\"PROGRAM_PANIC\">
        <name>
          <text>PROGRAM_PANIC</text>
        </name>
      </place>
      <place id=\"PROGRAM_START\">
        <name>
          <text>PROGRAM_START</text>
        </name>
        <initialMarking>
          <text>1</text>
        </initialMarking>
      </place>
      <transition id=\"BASIC_BLOCK_EMPTY\">
        <name>
          <text>BASIC_BLOCK_EMPTY</text>
        </name>
      </transition>
      <transition id=\"RETURN_main\">
        <name>
          <text>RETURN_main</text>
        </name>
      </transition>
      <arc source=\"BASIC_BLOCK_END_PLACE\" target=\"RETURN_main\" id=\"(BASIC_BLOCK_END_PLACE, RETURN_main)\">
        <name>
          <text>(BASIC_BLOCK_END_PLACE, RETURN_main)</text>
        </name>
        <inscription>
          <text>1</text>
        </inscription>
      </arc>
      <arc source=\"PROGRAM_START\" target=\"BASIC_BLOCK_EMPTY\" id=\"(PROGRAM_START, BASIC_BLOCK_EMPTY)\">
        <name>
          <text>(PROGRAM_START, BASIC_BLOCK_EMPTY)</text>
        </name>
        <inscription>
          <text>1</text>
        </inscription>
      </arc>
      <arc source=\"BASIC_BLOCK_EMPTY\" target=\"BASIC_BLOCK_END_PLACE\" id=\"(BASIC_BLOCK_EMPTY, BASIC_BLOCK_END_PLACE)\">
        <name>
          <text>(BASIC_BLOCK_EMPTY, BASIC_BLOCK_END_PLACE)</text>
        </name>
        <inscription>
          <text>1</text>
        </inscription>
      </arc>
      <arc source=\"RETURN_main\" target=\"PROGRAM_END\" id=\"(RETURN_main, PROGRAM_END)\">
        <name>
          <text>(RETURN_main, PROGRAM_END)</text>
        </name>
        <inscription>
          <text>1</text>
        </inscription>
      </arc>
    </page>
  </net>
</pnml>";

#[test]
fn minimal_program_generates_correct_dot_output_file() {
    assert_output_file(
        "./tests/sample_programs/minimal_program.rs",
        "dot",
        "./net.dot",
        MINIMAL_PROGRAM_DOT_OUTPUT,
    );
}

#[test]
fn minimal_program_generates_correct_lola_output_file() {
    assert_output_file(
        "./tests/sample_programs/minimal_program.rs",
        "lola",
        "./net.lola",
        MINIMAL_PROGRAM_LOLA_OUTPUT,
    );
}

#[test]
fn minimal_program_generates_correct_pnml_output_file() {
    assert_output_file(
        "./tests/sample_programs/minimal_program.rs",
        "pnml",
        "./net.pnml",
        MINIMAL_PROGRAM_PNML_OUTPUT,
    );
}
