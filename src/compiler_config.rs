//! Submodule for handling the rustc config.
//!
//! The Config struct is documented here:
//! <https://doc.rust-lang.org/stable/nightly-rustc/rustc_interface/interface/struct.Config.html>
//!
//! It includes command-line options as well as internal rustc options.

// Compiler crates need to be imported in this way because they are not published on crates.io.
// These crates are only available when using the nightly toolchain.
extern crate rustc_interface;
extern crate rustc_session;
extern crate rustc_span;

use rustc_errors::registry;
use rustc_session::config::{self, CheckCfg};
use rustc_span::source_map;

pub fn prepare_rustc_config(sysroot: std::path::PathBuf) -> rustc_interface::Config {
    rustc_interface::Config {
        opts: config::Options {
            maybe_sysroot: Some(sysroot),
            ..config::Options::default()
        },
        input: config::Input::Str {
            name: source_map::FileName::Custom("main.rs".to_string()),
            input: r#"
fn main() {
    let message = "Hello, World!";
    println!("{message}");
}
"#
            .to_string(),
        },
        crate_cfg: rustc_hash::FxHashSet::default(),
        crate_check_cfg: CheckCfg::default(),
        input_path: None,
        output_dir: None,
        output_file: None,
        file_loader: None,
        lint_caps: rustc_hash::FxHashMap::default(),
        parse_sess_created: None,
        register_lints: None,
        override_queries: None,
        make_codegen_backend: None,
        registry: registry::Registry::new(rustc_error_codes::DIAGNOSTICS),
    }
}
