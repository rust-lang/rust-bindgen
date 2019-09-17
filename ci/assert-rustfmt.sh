#!/usr/bin/env bash

set -xeu
cd "$(dirname "$0")/.."

rustup update nightly
rustup component add rustfmt --toolchain nightly

# Run `rustfmt` on the crate! If `rustfmt` can't make a long line shorter, it
# prints an error and exits non-zero, so tell it to kindly shut its yapper and
# make sure it doesn't cause us to exit this whole script non-zero.
rustup run nightly cargo fmt -- --check
