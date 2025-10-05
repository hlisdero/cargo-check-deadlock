//! Submodule for running the `LoLA` model checker.

use log::info;
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
pub fn check_deadlock(
    net_filepath: &std::path::PathBuf,
    witness_path: Option<&std::path::PathBuf>,
) -> bool {
    let mut cmd = initialize_command("lola", net_filepath, witness_path);
    let mut backup_cmd = initialize_command("./assets/lola", net_filepath, witness_path);

    let output = match cmd.output() {
        Ok(output) => output,
        Err(err) => {
            if err.kind() == std::io::ErrorKind::NotFound {
                info!("`lola` was not found in the $PATH. Please check that the program is installed and added to the $PATH");
            } else {
                panic!("There was an unknown error while executing `lola`: {err}");
            }
            backup_cmd
                .output()
                .expect("`lola` was not found in the $PATH nor in the `./assets/` folder")
        }
    };

    // For some reason `LoLA` only generates output to `stderr`.
    // Parse the answer to the reachability analysis and panic otherwise.
    let stderr_string =
        String::from_utf8(output.stderr).expect("Failed to convert the `lola` stderr to UTF-8");
    if stderr_string.contains("result: yes") {
        return true;
    }
    if stderr_string.contains("result: no") {
        return false;
    }
    panic!("Unknown output in command `lola`: {stderr_string}");
}

/// Initialize the command for calling the model checker with the right arguments
fn initialize_command(
    program_filepath: &str,
    net_filepath: &std::path::PathBuf,
    witness_path: Option<&std::path::PathBuf>,
) -> Command {
    let mut cmd = Command::new(program_filepath);

    cmd.arg(net_filepath)
        .arg("--formula=EF (DEADLOCK AND (PROGRAM_END = 0 AND PROGRAM_PANIC = 0))");

    if let Some(path) = witness_path {
        let path = path.to_string_lossy();
        cmd.arg(format!("--path={path}"));
    }

    cmd
}
