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
        .arg(net_filepath)
        .arg("--formula=EF (DEADLOCK AND (PROGRAM_END = 0 AND PROGRAM_PANIC = 0))")
        .output()
        .expect(
            "Could not execute `lola`. Please check that the program is installed and added to the $PATH",
        );

    // For some reason `LoLA` only generates output to `stderr`.
    // Parse the answer to the reachability analysis and panic otherwise.
    let stderr_string =
        String::from_utf8(output.stderr).expect("Failed to conver the `lola` stderr to UTF-8");
    if stderr_string.contains("result: yes") {
        return true;
    }
    if stderr_string.contains("result: no") {
        return false;
    }
    panic!("Unknown output in command `lola`: {stderr_string}");
}
