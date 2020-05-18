#!/usr/bin/env bash

set -xeu
cd "$(dirname "$0")/.."

# Note that `$BINDGEN_PROFILE` is never in quotes so that it expands to nothing
# (not even an empty string argument) when the variable is empty. This is
# necessary so we don't pass an unexpected flag to cargo.

export RUST_BACKTRACE=1

NO_DEFAULT_FEATURES=""
if [ ! -z $BINDGEN_NO_DEFAULT_FEATURES ]; then
  NO_DEFAULT_FEATURES=--no-default-features
fi

case "$BINDGEN_JOB" in
    "test")
        # Need rustfmt to compare the test expectations.
        TOOLCHAIN="nightly-$(curl https://rust-lang.github.io/rustup-components-history/$(rustup target list --installed | tail -1)/rustfmt)"
        rustup update "$TOOLCHAIN"
        rustup component add rustfmt --toolchain "$TOOLCHAIN"
        RUSTFMT="$(rustup which --toolchain "$TOOLCHAIN" rustfmt)"
        export RUSTFMT
        cargo test "$BINDGEN_PROFILE" $NO_DEFAULT_FEATURES --features "$BINDGEN_FEATURES"
        ./ci/assert-no-diff.sh
        ;;

    "msrv")
        # Test that the bindgen library can be built with the minimum supported Rust version
        # This test should not use Cargo.lock as it's ignored for library builds
        rm Cargo.lock
        # The MSRV below is also documented in README.md, please keep in sync
        rustup install 1.34.0
        cargo +1.34.0 build --lib $NO_DEFAULT_FEATURES --features "$BINDGEN_FEATURES"
        ;;

    "integration")
        cd ./bindgen-integration
        cargo test "$BINDGEN_PROFILE" $NO_DEFAULT_FEATURES --features "$BINDGEN_FEATURES"
        ;;

    "expectations")
        cd ./tests/expectations
        cargo test "$BINDGEN_PROFILE"
        ;;

    "misc")
        ./ci/assert-docs.sh
        ./ci/assert-warnings.sh
        ./ci/test-book.sh
        ./ci/no-includes.sh
        ./ci/assert-rustfmt.sh
        ;;

    "quickchecking")
        cd ./tests/quickchecking
        # TODO: Actually run quickchecks once `bindgen` is reliable enough.
        cargo test
        ;;

    *)
        echo "Error! Unknown \$BINDGEN_JOB: '$BINDGEN_JOB'"
        exit 1
esac
