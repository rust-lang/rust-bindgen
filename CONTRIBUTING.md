# Contributing to `servo/rust-bindgen`

Hi! We'd love to have your contributions! If you want help or mentorship, reach
out to us in a GitHub issue, or stop by #servo on irc.mozilla.org and introduce
yourself.

* [Code of Conduct](#coc)
* [Filing an Issue](#issue)
* [Building](#building)
* [Testing](#tests)
  * [Overview](#tests-overview)
  * [Running All Tests](#tests-all)
  * [Running a Single, Specific Test](#tests-one)
  * [Authoring New Tests](#tests-new)
* [Automatic Code Formatting](#formatting)
* [Debug Logging](#logs)

## Code of Conduct <span id="coc"/>

We abide by the [Rust Code of Conduct][coc] and ask that you do as well.

[coc]: https://www.rust-lang.org/en-US/conduct.html

## Filing an Issue <span id="issue"/>

Think you've found a bug? File an issue! To help us understand and reproduce the
issue, provide us with:

* A (preferrably reduced) C/C++ header file that reproduces the issue
* The `bindgen` flags used to reproduce the issue with the header file
* The expected `bindgen` output
* The actual `bindgen` output
* The [debugging logs](#logs) generated when running `bindgen` on this testcase

## Building <span id="building"/>

Build instructions are in the [README](./README.md).

Additionally, you may want to build and test with the `_docs` feature to ensure
that you aren't forgetting to document types and functions. CI will catch it if
you forget, but the turn around will be a lot slower ;)

```
$ cargo build --features "llvm_stable _docs"
```

## Testing <span id="tests"/>

### Overview <span id="tests-overview"/>

Input C/C++ test headers reside in the `tests/headers` directory. The expected
output rust bindings live in `tests/expectations/tests`; for example,
`tests/headers/my_header.h`'s expected generated rust bindings would be
`tests/expectations/tests/my_header.rs`.

The `tests/tools/run-bindgen.py` script runs `bindgen` on the test headers and
compares the results to the expectations.

### Running All Tests <span id="tests-all"/>

```
$ cargo test [--features llvm_stable]
```

This spawns a `tests/tools/run-bindgen.py` subprocess for each test header.

### Running a Single, Specific Test <span id="tests-one"/>

```
$ ./tests/tools/run-bindgen.py ./target/debug/bindgen ./tests/headers/some_header.h
```

To learn more about the options available with `run-bindgen.py`:

```
$ ./tests/tools/run-bindgen.py --help
```

### Authoring New Tests <span id="tests-new"/>

To add a new test header to the suite, simply put it in the `tests/headers`
directory. Next, run the `run-bindgen.py` script with the new test header, which
will generate the initial expected output rust bindings.

If your new test requires certain flags to be passed to `bindgen`, you can
specify them at the top of the test header, with a comment like this:

```c
// bindgen-flags: --enable-cxx-namespaces -- -std=c++14
```

If your new test requires `bindgen` to be built with certain features, you can
specify the required features at the top of the test header in a similar manner:

```c
// bingden-features: llvm_stable
```

Then verify the new rust bindings compile and its tests pass:

```
$ cargo test -p tests_expectations
```

## Automatic code formatting <span id="formatting"/>

There's a `rustfmt.toml` file in the repo. Ideally changes should be consistent
with the style, though that's not enforced right now.

[`rustfmt`](https://github.com/rust-lang-nursery/rustfmt) can catch and fix
automatically all the coding style issues it finds. In order to use it it
suffices to do:

```
$ cargo fmt
```

For it to work, you need to have `rustfmt` installed. To do so:

```
$ cargo install rustfmt
```

And ensure `~/.cargo/bin` is on your path.

## Debug Logging <span id="logs"/>

To help debug what `bindgen` is doing, you can define the environment variable
`RUST_LOG=bindgen` to get a bunch of debugging log spew.

```
$ RUST_LOG=bindgen ./target/debug/bindgen [flags...] ~/path/to/some/header.h
```

This logging can also be used when debugging failing tests under
`run-bindgen.py`:

```
$ RUST_LOG=bindgen ./tests/tools/run-bindgen.py ./target/debug/bindgen tests/headers/whatever.h
```
