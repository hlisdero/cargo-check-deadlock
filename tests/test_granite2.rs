use assert_cmd::prelude::*; // Add methods on commands
use assert_fs::prelude::*; // Used for creating temp files
use predicates::prelude::*; // Used for writing assertions
use std::process::Command; // Run programs

#[test]
fn file_does_not_exist() {
    let Ok(mut cmd) = Command::cargo_bin("granite2") else {
        panic!("Command not found");
    };

    cmd.arg("test/file/doesnt/exist")
        .arg("--output-format=pnml");
    cmd.assert()
        .failure()
        .stderr(predicate::str::contains("Could not open file"));
}

#[test]
fn output_format_is_not_valid() {
    let Ok(file) = assert_fs::NamedTempFile::new("valid_file.rs") else {
        panic!("Could not create temporary file for test");
    };
    file.write_str("fn main {}")
        .expect("Could not write test file contents");

    let Ok(mut cmd) = Command::cargo_bin("granite2") else {
            panic!("Command not found");
    };

    cmd.arg("valid_file.rs")
        .arg("--output-format=INVALID_FORMAT");
    cmd.assert().failure().stderr(predicate::str::contains(
        "[possible values: pnml, lola, dot]",
    ));
}
