#!/usr/bin/env bash

# Simple script to run `cargo clippy` with all the lints possible.
# Allow `clippy::missing-const-for-fn` because it causes false positives.

cargo clippy -- -Dclippy::pedantic -Dclippy::suspicious -Dclippy::nursery -Dclippy::complexity -Dclippy::perf -Dclippy::style -Dclippy::correctness -Aclippy::missing-const-for-fn
