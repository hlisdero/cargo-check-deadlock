//! Submodule for running the `LoLA` model checker.
use std::process::Command;

/// Checks for deadlock using the `LoLA` model checker.
/// Returns `true` if deadlock can be reached, otherwise returns `false`.
///
/// The CTL* formula used is `EF (DEADLOCK AND (PROGRAM_END = 0 AND PROGRAM_PANIC = 0))`.
/// This excludes the `PROGRAM_PANIC` and `PROGRAM_END` from being considered as deadlock states.
///
/// # Panics
///
/// If the command `lola` is not found, then the function panics.
/// If the command `lola` produces an extraneous output, then the function panics.
#[must_use]
pub fn check_deadlock(net_filepath: std::path::PathBuf) -> bool {
    let output = Command::new("lola")
        .arg("--formula=EF (DEADLOCK AND (PROGRAM_END = 0 AND PROGRAM_PANIC = 0))")
        .arg("-p")
        .arg(net_filepath)
        .output()
        .expect(
            "Could not execute `lola`. Please check that the program is installed and added to the $PATH",
        );

    // For some reason `LoLA` only generates output to `stderr`.
    // Parse the answer to the reachability analysis and panic otherwise.
    let stdout_string = String::from_utf8_lossy(&output.stderr);
    if stdout_string.find("result: yes").is_some() {
        return true;
    }
    if stdout_string.find("result: no").is_some() {
        return false;
    }
    panic!("Unknown output in command `lola`: {stdout_string}");
}
