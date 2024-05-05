#!/usr/bin/env bash

# Simple script to check if the new nightly version of rustc breaks the build

# Clean previous files to start from scratch
cargo clean

# Check if the project builds
cargo build

# Check if clippy is satisfied
cargo clippy -- -Dclippy::pedantic -Dclippy::suspicious -Dclippy::nursery -Dclippy::complexity -Dclippy::perf -Dclippy::style -Dclippy::correctness -Aclippy::missing-const-for-fn

# Check if the tests pass
cargo test
