use assert_cmd::prelude::*; // Add methods on commands
use assert_fs::prelude::*; // Used for creating temp files
use predicates::prelude::*; // Used for writing assertions
use std::process::Command; // Run programs

#[test]
fn file_does_not_exist() {
    let mut cmd = Command::cargo_bin("check-deadlock").expect("Command not found");

    cmd.arg("check-deadlock")
        .arg("test/file/doesnt/exist")
        .arg("--format=pnml");
    cmd.assert().failure().stderr(predicate::str::contains(
        "Source code file at test/file/doesnt/exist does not exist",
    ));
}

#[test]
fn output_folder_does_not_exist() {
    let file = assert_fs::NamedTempFile::new("valid_file.rs")
        .expect("Could not create temporary file for test");
    file.write_str("fn main {}")
        .expect("Could not write test file contents");

    let mut cmd = Command::cargo_bin("check-deadlock").expect("Command not found");
    cmd.arg("check-deadlock")
        .arg(file.path())
        .arg("--output-folder=test/folder/doesnt/exist");
    cmd.assert().failure().stderr(predicate::str::contains(
        "Output folder at test/folder/doesnt/exist does not exist",
    ));
}

#[test]
fn format_is_not_valid() {
    let file = assert_fs::NamedTempFile::new("valid_file.rs")
        .expect("Could not create temporary file for test");
    file.write_str("fn main() {}")
        .expect("Could not write test file contents");

    let mut cmd = Command::cargo_bin("check-deadlock").expect("Command not found");
    cmd.arg("check-deadlock")
        .arg(file.path())
        .arg("--format=INVALID_FORMAT");
    cmd.assert().failure().stderr(predicate::str::contains(
        "[possible values: pnml, lola, dot]",
    ));
}

#[test]
fn does_not_generate_output_by_default() {
    let file = assert_fs::NamedTempFile::new("valid_file.rs")
        .expect("Could not create temporary file for test");
    file.write_str("fn main() {}")
        .expect("Could not write test file contents");

    let mut cmd = Command::cargo_bin("check-deadlock").expect("Command not found");

    cmd.arg("check-deadlock")
        .arg(file.path())
        .arg("--filename=does_not_generate_output_by_default");
    cmd.assert()
        .success()
        .stdout(predicate::str::is_empty())
        .stderr(predicate::str::is_empty());

    if std::path::Path::new("./does_not_generate_output_by_default.dot").exists() {
        panic!("Should not generate a .dot file by default");
    }
    if std::path::Path::new("./does_not_generate_output_by_default.lola").exists() {
        panic!("Should not generate a .lola file by default");
    }
    if std::path::Path::new("./does_not_generate_output_by_default.pnml").exists() {
        panic!("Should not generate a .pnml file by default");
    }
}
