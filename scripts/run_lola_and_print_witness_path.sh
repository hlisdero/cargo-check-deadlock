#!/usr/bin/env bash

# Simple script to run `lola` and print the witness path that shows HOW the deadlock is reached.
# The witness path output is not captured by `granite2` and we need to resort to this workaround for now.
lola --formula="EF (DEADLOCK AND (PROGRAM_END = 0 AND PROGRAM_PANIC = 0))" --path $1