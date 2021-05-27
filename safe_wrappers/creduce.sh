#!/bin/bash

#clang -E temp.cpp > example.cpp

# Run from this directory
cd ${0%/*} || exit 1

cd ..
cargo build --release
cd safe_wrappers
creduce --n 1 test.sh example.cpp