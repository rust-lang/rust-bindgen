#!/usr/bin/env bash

set -xeu
cd "$(dirname "$0")/.."

# Note that `$BINDGEN_PROFILE` is never in quotes so that it expands to nothing
# (not even an empty string argument) when the variable is empty. This is
# necessary so we don't pass an unexpected flag to cargo.

export RUST_BACKTRACE=1

case "$BINDGEN_JOB" in
    "test")
        # Need rustfmt to compare the test expectations.
        rustup update nightly
        rustup run nightly cargo install -f rustfmt-nightly

        cargo test $BINDGEN_PROFILE --features "$BINDGEN_FEATURES"
        ./ci/assert-no-diff.sh
        ;;

    "integration")
        cd ./bindgen-integration
        cargo test $BINDGEN_PROFILE --features "$BINDGEN_FEATURES"
        ;;

    "expectations")
        cd ./tests/expectations
        cargo test $BINDGEN_PROFILE
        ;;

    "misc")
        ./ci/assert-docs.sh
        ./ci/test-book.sh
        ./ci/no-includes.sh
        # `rustfmt` isn't reaching a fixed point on bindgen
        # code... https://github.com/rust-lang-nursery/rustfmt/issues/1376
        # ./ci/assert-rustfmt.sh
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
