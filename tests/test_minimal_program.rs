mod common;
use common::assert_output_file;

const MINIMAL_PROGRAM_DOT_OUTPUT: &str = "\
digraph petrinet {
    PROGRAM_END [shape=\"circle\" xlabel=\"PROGRAM_END\" label=\"\"];
    PROGRAM_PANIC [shape=\"circle\" xlabel=\"PROGRAM_PANIC\" label=\"\"];
    PROGRAM_START [shape=\"circle\" xlabel=\"PROGRAM_START\" label=\"•\"];
    RUN_main [shape=\"box\" xlabel=\"RUN_main\" label=\"\"];
    PROGRAM_START -> RUN_main;
    RUN_main -> PROGRAM_END;
}
";

const MINIMAL_PROGRAM_LOLA_OUTPUT: &str = "\
PLACE
    PROGRAM_END,
    PROGRAM_PANIC,
    PROGRAM_START;

MARKING
    PROGRAM_START : 1;

TRANSITION RUN_main
  CONSUME
    PROGRAM_START : 1;
  PRODUCE
    PROGRAM_END : 1;
";

const MINIMAL_PROGRAM_PNML_OUTPUT: &str = "\
<?xml version=\"1.0\" encoding=\"utf-8\"?>
<pnml xmlns=\"http://www.pnml.org/version-2009/grammar/pnml\">
  <net id=\"net0\" type=\"http://www.pnml.org/version-2009/grammar/ptnet\">
    <page id=\"page0\">
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
      <transition id=\"RUN_main\">
        <name>
          <text>RUN_main</text>
        </name>
      </transition>
      <arc source=\"PROGRAM_START\" target=\"RUN_main\" id=\"(PROGRAM_START, RUN_main)\">
        <name>
          <text>(PROGRAM_START, RUN_main)</text>
        </name>
        <inscription>
          <text>1</text>
        </inscription>
      </arc>
      <arc source=\"RUN_main\" target=\"PROGRAM_END\" id=\"(RUN_main, PROGRAM_END)\">
        <name>
          <text>(RUN_main, PROGRAM_END)</text>
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
