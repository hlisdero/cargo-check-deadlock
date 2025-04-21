//! Submodule for handling the rustc config.
//!
//! The Config struct is documented here:
//! <https://doc.rust-lang.org/stable/nightly-rustc/rustc_interface/interface/struct.Config.html>
//!
//! It includes command-line options as well as internal rustc options.
//! The relevant parts in this case are the `sysroot` and the input file.
//!
//! See the rustc driver examples for other possible example configurations:
//! <https://rustc-dev-guide.rust-lang.org/rustc-driver.html>

use rustc_errors::registry;
use std::sync::atomic::AtomicBool;

pub static USING_INTERNAL_FEATURES: AtomicBool = AtomicBool::new(false);

pub fn prepare_rustc_config(
    sysroot: std::path::PathBuf,
    source_code_filepath: std::path::PathBuf,
) -> rustc_interface::Config {
    rustc_interface::Config {
        opts: rustc_session::config::Options {
            sysroot,
            ..rustc_session::config::Options::default()
        },
        crate_cfg: Vec::new(),
        crate_check_cfg: Vec::new(),
        input: rustc_session::config::Input::File(source_code_filepath),
        output_dir: None,
        output_file: None,
        hash_untracked_state: None,
        ice_file: None,
        file_loader: None,
        locale_resources: Vec::new(),
        lint_caps: rustc_hash::FxHashMap::default(),
        psess_created: None,
        register_lints: None,
        override_queries: None,
        make_codegen_backend: None,
        registry: registry::Registry::new(rustc_errors::codes::DIAGNOSTICS),
        using_internal_features: &USING_INTERNAL_FEATURES,
        expanded_args: Vec::new(),
    }
}
