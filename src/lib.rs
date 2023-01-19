//! Library code that implements the translation of Rust source code to a Petri net.
//!
//! It uses rustc's query system to navigate the AST.
//!
//! Adapted from the example in the rustc-dev guide:
//! <https://rustc-dev-guide.rust-lang.org/rustc-driver-interacting-with-the-ast.html>
//!
//! NOTE: For the library code to compile, you will need to first run the following:
//!   rustup component add rustc-dev llvm-tools-preview
//!
//! Tested with rustc 1.67.0-nightly (b3bc6bf31 2022-11-24)

// This feature gate is necessary to access the internal crates of the compiler.
// It has existed for a long time and since the compiler internals will never be stabilized,
// the situation will probably stay like this.
// <https://doc.rust-lang.org/unstable-book/language-features/rustc-private.html>
#![feature(rustc_private)]

// Compiler crates need to be imported in this way because they are not published on crates.io.
// These crates are only available when using the nightly toolchain.
// It suffices to declare them once to use their types and methods in the whole crate.
extern crate rustc_ast_pretty;
extern crate rustc_driver;
extern crate rustc_error_codes;
extern crate rustc_errors;
extern crate rustc_hash;
extern crate rustc_hir;
extern crate rustc_interface;
extern crate rustc_middle;
extern crate rustc_session;
extern crate rustc_span;

mod compiler_config;
mod counter;
mod error_handling;
mod naming;
mod stack;
mod sysroot;
mod translator;
mod utils;

use netcrab::petri_net::PetriNet;

/// Entry point for the translation of the Rust code to a Petri net.
///
/// # Errors
///
/// If the `sysroot` cannot be found, then an error is returned.
/// If the translation fails, then an error with the corresponding description is returned.
///
/// # Panics
///
/// If the global typing context `rustc_middle::ty::TyCtxt` cannot be found, then the function panics.
/// If the translation failed due to a bug, then the function panics.
pub fn run(source_file_path: std::path::PathBuf) -> Result<PetriNet, &'static str> {
    let sysroot = sysroot::get_from_rustc()?;
    let config = compiler_config::prepare_rustc_config(sysroot, source_file_path);
    let mut translation_result: Result<PetriNet, &'static str> = Err("Translation did not run");

    rustc_interface::run_compiler(config, |compiler| {
        compiler.enter(|queries| {
            // Get the global typing context (tcx: TyCtxt), which is the central data structure in the compiler.
            // <https://doc.rust-lang.org/stable/nightly-rustc/rustc_middle/ty/struct.TyCtxt.html>
            let Ok(query) = queries.global_ctxt() else {
                panic!("BUG: Unable to get the global typing context needed for the Translator");
            };

            // Run the translator as a query to the compiler.
            // <https://rustc-dev-guide.rust-lang.org/rustc-driver.html>
            query.enter(|tcx| {
                let mut translator = translator::Translator::new(tcx);
                translator.run();
                translation_result = translator.get_result();
            });
        });
    });

    translation_result
}
