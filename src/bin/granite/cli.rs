use clap::Parser;

use crate::cargo_result::CargoResult;
use crate::check_deadlock::Args;

#[derive(Debug, Parser)]
#[command(bin_name = "cargo", author, version, long_about = None)]
#[command(
    about = "Convert a Rust source code file into a Petri net and analyze the net to find deadlocks."
)]
pub enum Command {
    CheckDeadlock(Args),
}

impl Command {
    pub fn exec(self) -> CargoResult {
        match self {
            Self::CheckDeadlock(args) => args.exec(),
        }
    }
}

#[test]
fn verify_app() {
    use clap::CommandFactory;
    Command::command().debug_assert()
}
