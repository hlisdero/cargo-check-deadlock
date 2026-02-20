//! Library code that implements the translation of Rust source code to a Petri net.
//!
//! It uses rustc's query system to navigate the AST.
//!
//! Adapted from the example in the rustc-dev guide:
//! <https://rustc-dev-guide.rust-lang.org/rustc-driver-interacting-with-the-ast.html>
//!
//! NOTE: For the library code to compile, you will need to first run the following:
//! `rustup component add rustc-dev llvm-tools-preview`

// This feature gate is necessary to access the internal crates of the compiler.
// It has existed for a long time and since the compiler internals will never be stabilized,
// the situation will probably stay like this.
// <https://doc.rust-lang.org/unstable-book/language-features/rustc-private.html>
#![feature(rustc_private)]

// Compiler crates need to be imported in this way because they are not published on crates.io.
// These crates are only available when using the nightly toolchain.
// It suffices to declare them once to use their types and methods in the whole crate.
extern crate rustc_ast_pretty;
extern crate rustc_const_eval;
extern crate rustc_driver;
extern crate rustc_error_codes;
extern crate rustc_errors;
extern crate rustc_hir;
extern crate rustc_interface;
extern crate rustc_middle;
extern crate rustc_session;
extern crate rustc_span;

mod data_structures;
pub mod model_checker;
mod naming;
mod translator;
mod utils;

pub use data_structures::petri_net_interface::PetriNet;

use std::collections::HashMap;

/// The Config struct is documented here:
/// <https://doc.rust-lang.org/stable/nightly-rustc/rustc_interface/interface/struct.Config.html>
///
/// It includes command-line options as well as internal rustc options.
/// The relevant parts in this case is only the input file.
///
/// See the rustc driver examples for other possible example configurations:
/// <https://rustc-dev-guide.rust-lang.org/rustc-driver.html>
fn prepare_rustc_config(source_code_filepath: std::path::PathBuf) -> rustc_interface::Config {
    rustc_interface::Config {
        opts: rustc_session::config::Options::default(),
        crate_cfg: Vec::new(),
        crate_check_cfg: Vec::new(),
        input: rustc_session::config::Input::File(source_code_filepath),
        output_dir: None,
        output_file: None,
        hash_untracked_state: None,
        ice_file: None,
        file_loader: None,
        lint_caps: HashMap::default(),
        psess_created: None,
        register_lints: None,
        override_queries: None,
        make_codegen_backend: None,
        using_internal_features: &rustc_driver::USING_INTERNAL_FEATURES,
        extra_symbols: Vec::new(),
    }
}

/// Entry point for the translation of the Rust code to a Petri net.
///
/// Adapted from the [example in the rustc repo](https://github.com/rust-lang/rustc-dev-guide/blob/master/examples/rustc-interface-example.rs)
///
/// # Errors
///
/// If the `sysroot` cannot be found, then an error is returned.
/// If the translation fails, then an error with the corresponding description is returned.
///
/// # Panics
///
/// If the translation failed due to a bug, then the function panics.
pub fn run(source_code_filepath: std::path::PathBuf) -> Result<PetriNet, &'static str> {
    let config = prepare_rustc_config(source_code_filepath);
    let mut translation_result: Result<PetriNet, &'static str> = Err("Translation did not run");

    rustc_interface::run_compiler(config, |compiler| {
        // Parse the program and print the syntax tree.
        let krate = rustc_interface::passes::parse(&compiler.sess);
        // Analyze the program and inspect the types of definitions.
        rustc_interface::create_and_enter_global_ctxt(compiler, krate, |tcx| {
            let mut translator = translator::Translator::new(tcx);
            translator.run();
            translation_result = Ok(translator.get_result());
        });
    });

    translation_result
}
