# Property tests for `bindgen` with `quickchecking`

`quickchecking` generates random C headers to test `bindgen` 
using the [`quickcheck`][quickcheck] property testing crate. When testing 
`bindgen` with `quickchecking`, the generated header files are passed to 
`bindgen`'s `csmith-fuzzing/predicate.py` script. If that script fails, 
`quickchecking` panics, and you can report an issue containing the test case!

<!-- START doctoc generated TOC please keep comment here to allow auto update -->
<!-- DON'T EDIT THIS SECTION, INSTEAD RE-RUN doctoc TO UPDATE -->


- [Prerequisites](#prerequisites)
- [Running](#running)

<!-- END doctoc generated TOC please keep comment here to allow auto update -->

## Prerequisites

Requires `python3` to be in `$PATH`.

Many systems have `python3` by default but if your OS doesn't, its package 
manager may make it available:

```
$ sudo apt install python3
$ brew install python3
$ # Etc...
```

## Running

Run `quickchecking` binary to generate and test fuzzed C headers with 
`cargo run`. Additional configuration is exposed through the binary's CLI.

```
$ cargo run --bin=quickchecking -- -h
quickchecking 0.2.0
Bindgen property tests with quickcheck. Generate random valid C code and pass it to the csmith/predicate.py script

USAGE:
    quickchecking [OPTIONS]

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
    -c, --count <COUNT>    Count / number of tests to run. Running a fuzzed header through the predicate.py script can
                           take a long time, especially if the generation range is large. Increase this number if you're
                           willing to wait a while. [default: 2]
    -p, --path <PATH>      Optional. Preserve generated headers for inspection, provide directory path for header
                           output. [default: None] 
    -r, --range <RANGE>    Sets the range quickcheck uses during generation. Corresponds to things like arbitrary usize
                           and arbitrary vector length. This number doesn't have to grow much for execution time to
                           increase significantly. [default: 32]

```
[quickcheck]: https://github.com/BurntSushi/quickcheck
