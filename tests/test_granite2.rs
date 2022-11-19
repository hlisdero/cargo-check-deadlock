use anyhow::{Ok, Result};
use assert_cmd::prelude::*; // Add methods on commands
use assert_fs::prelude::*;
use predicates::prelude::*; // Used for writing assertions
use std::process::Command; // Run programs

#[test]
fn file_does_not_exist() -> Result<()> {
    let mut cmd = Command::cargo_bin("granite2")?;

    cmd.arg("test/file/doesnt/exist")
        .arg("--output-format=pnml");
    cmd.assert()
        .failure()
        .stderr(predicate::str::contains("Could not open file"));

    Ok(())
}

#[test]
fn output_format_is_not_valid() -> Result<()> {
    let file = assert_fs::NamedTempFile::new("valid_file.rs")?;
    file.write_str("fn main {}")?;

    let mut cmd = Command::cargo_bin("granite2")?;

    cmd.arg("valid_file.rs")
        .arg("--output-format=INVALID_FORMAT");
    cmd.assert().failure().stderr(predicate::str::contains(
        "[possible values: pnml, lola, dot]",
    ));

    Ok(())
}
