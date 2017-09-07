#!/usr/bin/env bash

# Usage:
#
#     ./tests/test-one.sh <fuzzy-name>
#
# Generate bindings for the first match of `./tests/headers/*<fuzzy-name>*`, use
# `rustc` to compile the bindings with unit tests enabled, and run the generated
# layout tests.

set -eu

if [ $# -ne 1 ]; then
    echo "Usage: $0 <fuzzy-name>"
    exit 1
fi

cd "$(dirname "$0")"
cd ..

export RUST_BACKTRACE=1

unique_fuzzy_file() {
    local pattern="$1"
    local results="$(find ./tests/headers -type f | egrep -i "$pattern")"
    local num_results=$(echo "$results" | wc -l)

    if [[ -z "$results" ]]; then
        >&2 echo "ERROR: no files found with pattern \"$pattern\""
        exit 1
    elif [[ "$num_results" -ne 1 ]]; then
        >&2 echo "ERROR: Expected exactly 1 result, got $num_results:"
        >&2 echo "$results"
        exit 1
    fi

    echo "$results"
}

TEST="$(unique_fuzzy_file "$1")"

BINDINGS=$(mktemp -t bindings.rs.XXXXXX)
TEST_BINDINGS_BINARY=$(mktemp -t bindings.XXXXXX)

FLAGS="$(grep "// bindgen-flags: " "$TEST" || echo)"
FLAGS="${FLAGS/\/\/ bindgen\-flags:/}"
# Prepend the default flags added in test.rs's `create_bindgen_builder`.
FLAGS="--rustfmt-bindings --with-derive-default --raw-line '' --raw-line '#![allow(dead_code, non_snake_case, non_camel_case_types, non_upper_case_globals)]' --raw-line '' $FLAGS"


eval ./target/debug/bindgen \
    "\"$TEST\"" \
    --emit-ir \
    --emit-ir-graphviz ir.dot \
    --emit-clang-ast \
    -o "\"$BINDINGS\"" \
    $FLAGS

rustup run nightly rustfmt "$BINDINGS" || true

dot -Tpng ir.dot -o ir.png

echo
echo "=== Input header ========================================================"
echo

cat "$TEST"

echo
echo "=== Generated bindings =================================================="
echo

cat "$BINDINGS"

echo
echo "=== Diff w/ expected bindings ==========================================="
echo

EXPECTED=${TEST/headers/expectations\/tests}
EXPECTED=${EXPECTED/.hpp/.rs}
EXPECTED=${EXPECTED/.h/.rs}

rustup run nightly rustfmt "$EXPECTED" || true

# Don't exit early if there is a diff.
diff -U8 "$EXPECTED" "$BINDINGS" || true

echo
echo "=== Building bindings ==================================================="
echo

rustc --test -o "$TEST_BINDINGS_BINARY" "$BINDINGS" --crate-name bindgen_test_one

echo
echo "=== Testing bindings ===================================================="
echo

"$TEST_BINDINGS_BINARY"
