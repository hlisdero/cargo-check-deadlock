// This feature gate is necessary to access the internal crates of the compiler.
// It has existed for a long time and since the compiler internals will never be stabilized,
// the situation will probably stay like this.
// <https://doc.rust-lang.org/unstable-book/language-features/rustc-private.html>
#![feature(rustc_private)]

mod cargo_result;
mod check_deadlock;
mod cli;
mod output_format;

use clap::Parser;

use cargo_result::CargoResult::{
    DeadlockAnalysis, OutputFolderNotFound, OutputGenerationError, SimpleTranslation,
    SourceFileNotFound, TranslationError,
};

fn main() {
    let args = cli::Command::parse();

    match args.exec() {
        SourceFileNotFound(err_str) => {
            eprintln!("{err_str}");
            std::process::exit(1);
        }
        OutputFolderNotFound(err_str) => {
            eprintln!("{err_str}");
            std::process::exit(2);
        }
        TranslationError(err_str) => {
            eprintln!("{err_str}");
            std::process::exit(3);
        }
        OutputGenerationError(err_str) => {
            eprintln!("{err_str}");
            std::process::exit(4);
        }
        DeadlockAnalysis(message) => {
            println!("Result: {message}");
        }
        SimpleTranslation => {}
    }
}
