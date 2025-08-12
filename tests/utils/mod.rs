use assert_cmd::prelude::*; // Add methods on commands
use predicates::prelude::*; // Used for writing assertions
use std::path::PathBuf;
use std::process::Command; // Run programs

/// Asserts that the contents of the output files correspond to the expected file contents
/// after running `cargo-check-deadlock` on the given source code file.
/// Checks all formats: LoLA, DOT and PNML.
///
/// # Panics
///
/// If the command `cargo-check-deadlock` is not found, then the function panics.
/// If the command `cargo-check-deadlock` fails to complete execution successfully, then the function panics.
/// If the output file cannot be opened, then the function panics.
/// If the output file contents cannot be read, then the function panics.
#[allow(dead_code)]
pub fn assert_output_files(source_code_file: &str, output_folder: &str) {
    let mut cmd = Command::cargo_bin("cargo-check-deadlock").expect("Command not found");

    // Current workdir is always the project root folder
    cmd.arg("check-deadlock")
        .arg(source_code_file)
        .arg(format!("--output-folder={output_folder}"))
        .arg("--dot")
        .arg("--pnml")
        .arg("--filename=test")
        .arg("--skip-analysis");

    cmd.assert().success();

    for extension in ["lola", "dot", "pnml"] {
        let output_path = PathBuf::from(format!("{output_folder}test.{extension}"));
        let expected_output_path = PathBuf::from(format!("{output_folder}net.{extension}"));

        let file_contents =
            std::fs::read_to_string(&output_path).expect("Could not read output file to string");

        let expected_file_contents = std::fs::read_to_string(&expected_output_path)
            .expect("Could not read file with expected contents to string");

        if file_contents != expected_file_contents {
            panic!(
                "The contents of {} do not match the contents of {}",
                output_path.to_string_lossy(),
                expected_output_path.to_string_lossy()
            );
        }

        std::fs::remove_file(output_path).expect("Could not delete output file");
    }
}

/// Asserts that the result of running the model checker  `LoLA` matches the expected result
/// (program has a deadlock or deadlock-free) after running `cargo-check-deadlock` on the given source code file.
///
/// # Panics
///
/// If the command `cargo-check-deadlock` is not found, then the function panics.
/// If the command `cargo-check-deadlock` fails to complete execution successfully, then the function panics.
#[allow(dead_code)]
pub fn assert_lola_result(
    source_code_file: &str,
    output_folder: &str,
    output_should_have_deadlock: bool,
) {
    let mut cmd = Command::cargo_bin("cargo-check-deadlock").expect("Command not found");

    // Current workdir is always the project root folder
    cmd.arg("check-deadlock")
        .arg(source_code_file)
        .arg(format!("--output-folder={output_folder}"))
        .arg("--filename=deadlock_test");

    if output_should_have_deadlock {
        cmd.assert().success().stdout(predicate::str::contains(
            "Result: Deadlock can be reached according to the model checker `LoLA`",
        ));
    } else {
        cmd.assert().success().stdout(predicate::str::contains(
            "Result: The program is deadlock-free according to the model checker `LoLA`",
        ));
    }
    // Delete the output file
    let output_filename = PathBuf::from(format!("{output_folder}deadlock_test.lola"));
    std::fs::remove_file(output_filename).expect("Could not delete output file");
}

/// This macro generates the test code for the three supported file formats.
/// It saves a considerable amount of boilerplate.
///
/// Receives the relative path from the root folder of the repository
/// to the source code of the program to be tested.
/// Receives also the relative path from the root folder of the repository
/// to the folder where the expected results are to be found.
///
/// The main idea for the implementation was taken from:
/// <https://doc.rust-lang.org/rust-by-example/macros/dry.html>
#[allow(unused_macros)]
macro_rules! generate_tests_for_example_program {
    ($fn_name:ident, $program_path:literal, $result_folder_path:literal) => {
        #[test]
        fn $fn_name() {
            super::utils::assert_output_files($program_path, $result_folder_path);
        }
    };
}

/// This macro generates the test code for the three supported file formats.
/// It saves a considerable amount of boilerplate.
///
/// Receives the relative path from the root folder of the repository
/// to the source code of the program to be tested.
/// Receives also the relative path from the root folder of the repository
/// to the folder where the expected results are to be found.
///
/// The main idea for the implementation was taken from:
/// <https://doc.rust-lang.org/rust-by-example/macros/dry.html>
#[allow(unused_macros)]
macro_rules! generate_lola_tests_for_example_program {
    ($fn_name:ident, $program_path:literal, $result_folder_path:literal, $expected_result:expr) => {
        #[test]
        fn $fn_name() {
            super::utils::assert_lola_result($program_path, $result_folder_path, $expected_result);
        }
    };
}

// Exports the previously defined macro.
// For the idea for the re-export, see:
// <https://stackoverflow.com/questions/26731243/how-do-i-use-a-macro-across-module-files#31749071>
// A warning is generated if some test file does NOT use this function.
// That is because each test is compiled as an independent crate.
// See more details here: <https://stackoverflow.com/a/67902444>
#[allow(unused_imports)]
pub(crate) use generate_lola_tests_for_example_program;
#[allow(unused_imports)]
pub(crate) use generate_tests_for_example_program;
