//! Submodule for getting the sysroot path.
//!
//! A "sysroot" is used by the compiler to hold any resources it may need.
//! For example: pre-compiled versions of the standard library, binaries
//! for rustc and related tools, and documentation.
//!
//! For our purposes it should point to the nightly toolchain, e.g,
//! `.rustup/toolchains/nightly-x86_64-unknown-linux-gnu`
//!
//! The previous implementation was based on miri:
//! <https://github.com/rust-lang/miri/blob/master/src/bin/miri.rs>
//!
//! miri needs to handle the different stages of compilation of rustc,
//! as it is compiled with clippy, rustfmt and other tools bundled with rustc.
//! This makes the logic more complex than necessary for our tool.

use log::info;
use std::path::PathBuf;
use std::process::Command;
use std::str;

/// Gets the current sysroot from running the rustc compiler.
pub fn get_from_rustc() -> Result<PathBuf, &'static str> {
    // Run rustc --print=sysroot and get the stdout.
    let Ok(out) = Command::new("rustc")
        .arg("--print=sysroot")
        .current_dir(".")
        .output()
        else {
        return Err("Could not run rustc to get the sysroot: Make sure you can run `rustc --print=sysroot` in a terminal");
    };
    // Convert the stdout to a str.
    let Ok(sysroot) = str::from_utf8(&out.stdout) else {
        return Err("Could not parse stdout to get the sysroot: Make sure you can run `rustc --print=sysroot` in a terminal");
    };
    info!("Found sysroot: {}", sysroot.trim());
    Ok(PathBuf::from(sysroot.trim()))
}
