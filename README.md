# Granite2

Find Deadlocks in Rust code with Petri net model checking.

- used rust nightly can be found in the [rust-toolchain file](https://doc.rust-lang.org/nightly/edition-guide/rust-2018/rustup-for-managing-rust-versions.html#managing-versions)
- rustc-dev component is needed ``rustup toolchain install [nightly-x-y-z] --component rustc-dev``
- also the linker has to know about the lib folder from the sysroot fiting the toolchain.
- some useful scripts can be found in the script folder. This includes:
  - an install script for the model checker LoLa
  - a run script that can translate programs from ``./tests/sample_programs``
  - and a script that can analyze the output

## Acknowledgments

Based on the original work by Tom Meyer found in <https://github.com/Skasselbard/Granite>
