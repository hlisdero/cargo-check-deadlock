#!/usr/bin/env bash

# Simple script to run `lola` and print the witness path that shows HOW the deadlock is reached.
# The witness path output is not captured by `granite` and we need to resort to this workaround for now.

if ! command -v lola &>/dev/null; then
  echo "lola could not be found in \$PATH. Looking for the version included in the repository..."
  if [ -f .asserts/lola ]; then
    echo "lola found in ./assets."
    .asserts/lola --formula="EF (DEADLOCK AND (PROGRAM_END = 0 AND PROGRAM_PANIC = 0))" --path $1
  else
    echo "lola could not be found in ./assets"
    exit 1
  fi
else
  echo "lola found in \$PATH."
  lola --formula="EF (DEADLOCK AND (PROGRAM_END = 0 AND PROGRAM_PANIC = 0))" --path $1
fi
