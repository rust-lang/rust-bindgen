#!/usr/bin/bash
../csmith-fuzzing/predicate.py \
    --expect-bindgen-fail \
    --bindgen-grep "panicked" \
    ./example.cpp