#!/usr/bin/env bash

set -xeu
cd "$(dirname "$0")/.."

TOOLCHAIN="nightly-$(curl https://rust-lang.github.io/rustup-components-history/$(rustup target list --installed | tail -1)/rustfmt)"
rustup update "$TOOLCHAIN"
rustup component add rustfmt --toolchain "$TOOLCHAIN"

# Run `rustfmt` on the crate! If `rustfmt` can't make a long line shorter, it
# prints an error and exits non-zero, so tell it to kindly shut its yapper and
# make sure it doesn't cause us to exit this whole script non-zero.
rustup run "$TOOLCHAIN" cargo fmt -- --check
