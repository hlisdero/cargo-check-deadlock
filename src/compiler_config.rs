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
use rustc_session::config::{self, CheckCfg};

pub fn prepare_rustc_config(
    sysroot: std::path::PathBuf,
    source_file_path: std::path::PathBuf,
) -> rustc_interface::Config {
    rustc_interface::Config {
        opts: config::Options {
            maybe_sysroot: Some(sysroot),
            ..config::Options::default()
        },
        crate_cfg: rustc_hash::FxHashSet::default(),
        crate_check_cfg: CheckCfg::default(),
        input: config::Input::File(source_file_path),
        output_dir: None,
        output_file: None,
        file_loader: None,
        locale_resources: &[],
        lint_caps: rustc_hash::FxHashMap::default(),
        parse_sess_created: None,
        register_lints: None,
        override_queries: None,
        make_codegen_backend: None,
        registry: registry::Registry::new(rustc_error_codes::DIAGNOSTICS),
    }
}
