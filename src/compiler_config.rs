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

use std::collections::HashMap;

pub fn prepare_rustc_config(
    sysroot: std::path::PathBuf,
    source_code_filepath: std::path::PathBuf,
) -> rustc_interface::Config {
    let sysroot = rustc_session::config::Sysroot::new(Some(sysroot));
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
        lint_caps: HashMap::default(),
        psess_created: None,
        register_lints: None,
        override_queries: None,
        make_codegen_backend: None,
        using_internal_features: &rustc_driver::USING_INTERNAL_FEATURES,
        extra_symbols: Vec::new(),
    }
}
