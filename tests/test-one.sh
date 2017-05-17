#!/usr/bin/env bash

# Usage:
#
#     ./tests/test-one.sh <fuzzy-name>
#
# Generate bindings for the first match of `./tests/headers/*<fuzzy-name>*`, use
# `rustc` to compile the bindings with unit tests enabled, and run the generated
# layout tests.

set -eu

cd $(dirname $0)
cd ..

export RUST_BACKTRACE=1

# Grab the first match
TEST=$(find ./tests/headers -type f -iname "*$1*" | head -n 1)

BINDINGS=$(mktemp -t bindings.rs.XXXXXX)
TEST_BINDINGS_BINARY=$(mktemp -t bindings.XXXXXX)

FLAGS="$(grep "// bindgen-flags: " "$TEST" || echo)"
FLAGS="${FLAGS/\/\/ bindgen\-flags:/}"

eval ./target/debug/bindgen \
    "\"$TEST\"" \
    --emit-ir \
    --emit-ir-graphviz ir.dot \
    --emit-clang-ast \
    -o "\"$BINDINGS\"" \
    $FLAGS

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
