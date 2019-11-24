#!/usr/bin/env bash

set -xeu
cd "$(dirname "$0")/.."

cargo rustc --lib --features "$BINDGEN_FEATURES" -- -Dwarnings
