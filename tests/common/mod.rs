use assert_cmd::prelude::*; // Add methods on commands
use std::process::Command; // Run programs

/// Asserts that the contents of the given output file correspond to the expected file contents
/// after running `granite2` on the given source code file.
///
/// # Panics
///
/// If the command `granite2` is not found, then the function panics.
/// If the output file cannot be opened, then the function panics.
/// If the output file contents cannot be read, then the function panics.
pub fn assert_output_file(
    source_code_file: &str,
    output_format: &str,
    output_filename: &str,
    expected_contents_filename: &str,
) {
    let mut cmd = Command::cargo_bin("granite2").expect("Command not found");

    // Current workdir is always the project root folder
    cmd.arg(source_code_file)
        .arg(format!("--output-format={output_format}"));
    cmd.assert().success();

    let file_contents =
        std::fs::read_to_string(output_filename).expect("Could not read output file to string");

    let expected_file_contents = std::fs::read_to_string(expected_contents_filename)
        .expect("Could not read file with expected contents to string");

    assert_eq!(file_contents, expected_file_contents);
    std::fs::remove_file(output_filename).expect("Could not delete output file");
}
