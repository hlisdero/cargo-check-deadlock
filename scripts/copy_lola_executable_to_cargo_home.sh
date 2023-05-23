#!/usr/bin/env bash

# Simple script to install the `lola` executable in $CARGO_HOME.
# If CARGO_HOME is not set, use the default $HOME/.cargo/
# <https://doc.rust-lang.org/cargo/guide/cargo-home.html>

if command -v lola &>/dev/null; then
  echo "lola found in \$PATH. No need to install it."
  exit 0
fi

echo "lola is not in the \$PATH."
if [ ! -f ./assets/lola ]; then
  echo "lola could not be found in ./assets"
  echo "Please make sure to run the script from the root folder of the project."
  exit 1
fi

echo "lola found in ./assets"
if [[ -z "$CARGO_HOME" ]]; then
  echo "\$CARGO_HOME is undefined. Installing lola in \$HOME/.cargo/bin ..."
  cp ./assets/lola $HOME/.cargo/bin
else
  echo "\$CARGO_HOME is set. Installing lola in the bin subfolder..."
  cp ./assets/lola $CARGO_HOME/bin
fi
echo "lola is now installed"
