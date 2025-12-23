#[expect(
    deprecated,
    reason = "cargo_bin is deprecated, cargo_bin! is not, `use` does not differenciate them"
)]
use assert_cmd::cargo::cargo_bin;
use assert_cmd::prelude::*; // Add methods on commands
use assert_fs::prelude::*; // Used for creating temp files
use predicates::prelude::*; // Used for writing assertions
use std::process::Command; // Run programs

#[test]
fn file_does_not_exist() {
    let mut cmd = Command::new(cargo_bin!("cargo-check-deadlock"));

    cmd.arg("check-deadlock")
        .arg("test/file/doesnt/exist")
        .arg("--pnml");
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

    let mut cmd = Command::new(cargo_bin!("cargo-check-deadlock"));
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

    let mut cmd = Command::new(cargo_bin!("cargo-check-deadlock"));
    cmd.arg("check-deadlock").arg(file.path()).arg("--csv");
    cmd.assert().failure().stderr(predicate::str::contains(
        "unexpected argument '--csv' found",
    ));
}

#[test]
fn generates_lola_output_by_default() {
    let file = assert_fs::NamedTempFile::new("valid_file.rs")
        .expect("Could not create temporary file for test");
    file.write_str("fn main() {}")
        .expect("Could not write test file contents");

    let mut cmd = Command::new(cargo_bin!("cargo-check-deadlock"));
    cmd.arg("check-deadlock")
        .arg(file.path())
        .arg("--filename=does_not_generate_output_by_default")
        .arg("--skip-analysis");
    cmd.assert()
        .success()
        .stdout(predicate::str::is_empty())
        .stderr(predicate::str::is_empty());
    // Check that the default output exists
    if !std::path::Path::new("./does_not_generate_output_by_default.lola").exists() {
        panic!("Should generate a .lola file by default");
    }
    std::fs::remove_file("./does_not_generate_output_by_default.lola")
        .expect("Could not delete output file");

    // Check that the other formats were not generated
    if std::path::Path::new("./does_not_generate_output_by_default.dot").exists() {
        panic!("Should not generate a .dot file by default");
    }
    if std::path::Path::new("./does_not_generate_output_by_default.pnml").exists() {
        panic!("Should not generate a .pnml file by default");
    }
}
