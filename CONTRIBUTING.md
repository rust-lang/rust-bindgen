# Contributing to `bindgen`

Hi! We'd love to have your contributions! If you want help or mentorship, reach
out to us in a GitHub issue, or stop by
[#rust on chat.mozilla.org](https://chat.mozilla.org/#/room/#rust:mozilla.org)
and introduce yourself.

<!-- START doctoc generated TOC please keep comment here to allow auto update -->
<!-- DON'T EDIT THIS SECTION, INSTEAD RE-RUN doctoc TO UPDATE -->

- [Code of Conduct](#code-of-conduct)
- [Filing an Issue](#filing-an-issue)
- [Looking to Start Contributing to `bindgen`?](#looking-to-start-contributing-to-bindgen)
- [Prerequisites](#prerequisites)
  - [`rustfmt` / `cargo fmt`](#rustfmt--cargo-fmt)
- [Building](#building)
- [Testing](#testing)
  - [Overview](#overview)
  - [Testing Bindings Generation](#testing-bindings-generation)
  - [Testing Generated Bindings](#testing-generated-bindings)
  - [Testing a Single Header's Bindings Generation and Compiling its Bindings](#testing-a-single-headers-bindings-generation-and-compiling-its-bindings)
  - [Authoring New Tests](#authoring-new-tests)
  - [Test Expectations and `libclang` Versions](#test-expectations-and-libclang-versions)
  - [Integration Tests](#integration-tests)
  - [Fuzzing `bindgen` with `csmith`](#fuzzing-bindgen-with-csmith)
  - [Property tests for `bindgen` with `quickchecking`](#property-tests-for-bindgen-with-quickchecking)
- [Code Overview](#code-overview)
  - [Implementing new options using `syn`](#implementing-new-options-using-syn)
- [Pull Requests and Code Reviews](#pull-requests-and-code-reviews)
- [Generating Graphviz Dot Files](#generating-graphviz-dot-files)
- [Debug Logging](#debug-logging)
- [Using `creduce` to Minimize Test Cases](#using-creduce-to-minimize-test-cases)
  - [Getting `creduce`](#getting-creduce)
  - [Isolating Your Test Case](#isolating-your-test-case)
  - [Writing a Predicate Script](#writing-a-predicate-script)
- [Cutting a new bindgen release](#cutting-a-new-bindgen-release)
  - [Updating the changelog](#updating-the-changelog)
  - [Merge to `main`](#merge-to-main)
  - [Tag and publish](#tag-and-publish)
  - [Create a new release on GitHub](create-a-new-relese-on-github)

<!-- END doctoc generated TOC please keep comment here to allow auto update -->

## Code of Conduct

We abide by the [Rust Code of Conduct][coc] and ask that you do as well.

[coc]: https://www.rust-lang.org/policies/code-of-conduct

## Filing an Issue

Think you've found a bug? File an issue! To help us understand and reproduce the
issue, provide us with:

* A (preferably reduced) C/C++ header file that reproduces the issue
* The `bindgen` flags used to reproduce the issue with the header file
* The expected `bindgen` output
* The actual `bindgen` output
* The [debugging logs](#debug-logging) generated when running `bindgen` on this testcase

## Looking to Start Contributing to `bindgen`?

* [Issues labeled "easy"](https://github.com/rust-lang/rust-bindgen/issues?q=is%3Aopen+is%3Aissue+label%3AE-easy)
* [Issues labeled "less easy"](https://github.com/rust-lang/rust-bindgen/issues?q=is%3Aopen+is%3Aissue+label%3AE-less-easy)
* [Issues labeled "help wanted"](https://github.com/rust-lang/rust-bindgen/labels/help%20wanted)
* Still can't find something to work on? [Drop a comment here](https://github.com/rust-lang/rust-bindgen/issues/747)

## Prerequisites

### `rustfmt` / `cargo fmt`

We use `nightly` channel for `rustfmt`,
so please set the appropriate setting in your editor/IDE for that.

For rust-analyzer, you can set `rustfmt.extraArgs = ['+nightly']`.

To check via command line, you can run `cargo +nightly fmt --check`.

## Building

To build the `bindgen` library and the `bindgen` executable:

```
$ cargo build
```

If you installed multiple versions of llvm, it may not be able to locate the
latest version of libclang. In that case, you may want to either uninstall other
versions of llvm, or specify the path of the desired libclang explicitly:

```
$ export LIBCLANG_PATH=path/to/clang-9.0/lib
```

## Testing

### Overview

Input C/C++ test headers reside in the `bindgen-tests/tests/headers` directory. Expected
output Rust bindings live in `bindgen-tests/tests/expectations/tests`. For example,
`bindgen-tests/tests/headers/my_header.h`'s expected generated Rust bindings would be
`bindgen-tests/tests/expectations/tests/my_header.rs`.

There are also some integration tests in the `./bindgen-integration` crate, which uses `bindgen` to
generate bindings to some C++ code, and then uses the bindings, asserting that
values are what we expect them to be, both on the Rust and C++ side.

The generated and expected bindings are formatted with [prettyplease] before they are
compared. It is a default (but optional) dependency of `bindgen`,
so be sure to keep that in mind
(if you built `bindgen` with the `--no-default-features` option of Cargo).
Note also that `rustfmt` formatting is disabled for the `bindgen-tests/tests/expectations/`
directory tree, which helps avoid failing ui tests.

Note: running `cargo test` from the root directory of `bindgen`'s repository does not
automatically test the generated bindings or run the integration tests.
These steps must be performed manually when needed.


### Testing Bindings Generation

To regenerate bindings from the corpus of test headers in `bindgen-tests/tests/headers` and
compare them against the expected bindings in `bindgen-tests/tests/expectations/tests`, run:

```
$ cargo test
```

As long as you aren't making any changes to `bindgen`'s output, running this
should be sufficient to test your local modifications.

You may set the `BINDGEN_OVERWRITE_EXPECTED` environment variable to overwrite
the expected bindings with `bindgen`'s current output:

```
$ BINDGEN_OVERWRITE_EXPECTED=1 cargo test
```

If you set the BINDGEN_TESTS_DIFFTOOL environment variable, `cargo test` will
execute $BINDGEN_TESTS_DIFFTOOL /path/of/expected/output /path/of/actual/output
when the expected output differs from the actual output. You can use this to
hand check differences by setting it to e.g. "meld" (assuming you have meld
installed).

If you're not changing command line arguments, you may want to set
`BINDGEN_DISABLE_ROUNDTRIP_TEST` to avoid a lot of tests for round-tripping of
those.

### Testing Generated Bindings

If your local changes are introducing expected modifications in the
`bindgen-tests/tests/expectations/tests/*` bindings files, then you should test that the
generated bindings files still compile, and that their struct layout tests still
pass. Also, run the integration tests (see below).

You can do this with these commands:

```
$ cd bindgen-tests/tests/expectations
$ cargo test
```

### Testing a Single Header's Bindings Generation and Compiling its Bindings

Note: You will need to install [Graphviz](https://graphviz.org/) since that
is a dependency for running `test-one.sh`.

Sometimes it's useful to work with one test header from start (generating
bindings for it) to finish (compiling the bindings and running their layout
tests). This can be done with the `bindgen-tests/tests/test-one.sh` script. It supports fuzzy
searching for test headers. For example, to test
`tests/headers/what_is_going_on.hpp`, execute this command:

```
$ ./bindgen-tests/tests/test-one.sh going
```

Note that `test-one.sh` does not recompile `bindgen`, so if you change the code,
you'll need to rebuild it before running the script again.

### Authoring New Tests

To add a new test header to the suite, simply put it in the `bindgen-tests/tests/headers`
directory. Next, run `bindgen` to generate the initial expected output Rust
bindings. Put those in `bindgen-tests/tests/expectations/tests`.

If your new test requires certain flags to be passed to `bindgen`, you can
specify them at the top of the test header, with a comment like this:

`new_test_header.hpp`:

```c
// bindgen-flags: --enable-cxx-namespaces -- -std=c++14
```

Then verify the new Rust bindings compile and pass their layout tests:

```
$ cd bindgen-tests/tests/expectations
$ cargo test new_test_header
```

### Test Expectations and `libclang` Versions

If a test generates different bindings across different `libclang` versions (for
example, because we take advantage of better/newer APIs when possible), then you
can add multiple test expectations, one for each supported `libclang`
version. Instead of having a single `bindgen-tests/tests/expectations/tests/my_test.rs` file,
add each of:

* `bindgen-tests/tests/expectations/tests/libclang-16/my_test.rs`
* `bindgen-tests/tests/expectations/tests/libclang-9/my_test.rs`

If you need to update the test expectations for a test file that generates
different bindings for different `libclang` versions, you *don't* need to have
many versions of `libclang` installed locally. Just make a work-in-progress pull
request, and then when CI fails, it will log a diff of the
expectations. Use the diff to patch the appropriate expectation file locally and
then update your pull request.

Usually, `bindgen`'s test runner can infer which version of `libclang` you
have. If for some reason it can't, you can force a specific `libclang` version
to check the bindings against with a cargo feature:

```
$ cargo test --features __testing_only_libclang_$VERSION
```

depending on which version of `libclang` you have installed.

### Integration Tests

The `./bindgen-integration` crate uses `bindgen` to
generate bindings to some C++ code, and then uses the bindings, asserting that
values are what we expect them to be, both on the Rust and C++ side.

To run the integration tests, issue the following:

```
$ cd bindgen-integration
$ cargo test
```

### Fuzzing `bindgen` with `csmith`

We <3 finding hidden bugs and the people who help us find them! One way to help
uncover hidden bugs is by running `csmith` to generate random headers to test
`bindgen` against.

See [./csmith-fuzzing/README.md](./csmith-fuzzing/README.md) for details.

### Property tests for `bindgen` with `quickchecking`

The `tests/quickchecking` crate generates property tests for `bindgen`.
From the crate's directory you can run the tests with `cargo run`. For details
on additional configuration including how to preserve / inspect the generated
property tests, see
[./tests/quickchecking/README.md](./tests/quickchecking/README.md).

## Code Overview

`bindgen` takes C and C++ header files as input and generates corresponding Rust
`#[repr(C)]` type definitions and `extern` foreign function declarations.

First, we use `libclang` to parse the input headers. See `src/clang.rs` for our
Rust-y wrappers over the raw C `libclang` API that the `clang-sys` crate
exposes. We walk over `libclang`'s AST and construct our own internal
representation (IR).  The `ir` module and submodules (`src/ir/*`) contain the IR
type definitions and `libclang` AST into IR parsing code.

The umbrella IR type is the `Item`. It contains various nested `enum`s that let
us drill down and get more specific about the kind of construct that we're
looking at. Here is a summary of the IR types and their relationships:

* `Item` contains:
    * An `ItemId` to uniquely identify it.
    * An `ItemKind`, which is one of:
        * A `Module`, which is originally a C++ namespace and becomes a Rust
          module. It contains the set of `ItemId`s of `Item`s that are defined
          within it.
        * A `Type`, which contains:
            * A `Layout`, describing the type's size and alignment.
            * A `TypeKind`, which is one of:
                * Some integer type.
                * Some float type.
                * A `Pointer` to another type.
                * A function pointer type, with `ItemId`s of its parameter types
                  and return type.
                * An `Alias` to another type (`typedef` or `using X = ...`).
                * A fixed size `Array` of `n` elements of another type.
                * A `Comp` compound type, which is either a `struct`, `class`,
                  or `union`. This is potentially a template definition.
                * A `TemplateInstantiation` referencing some template definition
                  and a set of template argument types.
                * Etc...
        * A `Function`, which contains:
            * An ABI
            * A mangled name
            * a `FunctionKind`, which describes whether this function is a plain
              function, method, static method, constructor, destructor, etc.
            * The `ItemId` of its function pointer type.
        * A `Var` representing a static variable or `#define` constant, which
          contains:
            * Its type's `ItemId`
            * Optionally, a mangled name
            * Optionally, a value
    * An optional `clang::SourceLocation` that holds the first source code
      location where the `Item` was encountered.

The IR forms a graph of interconnected and inter-referencing types and
functions. The `ir::traversal` module provides IR graph traversal
infrastructure: edge kind definitions (base member vs field type vs function
parameter, etc...), the `Trace` trait to enumerate an IR thing's outgoing edges,
various traversal types.

After constructing the IR, we run a series of analyses on it. These analyses do
everything from allocate logical bitfields into physical units, compute for
which types we can `#[derive(Debug)]`, to determining which implicit template
parameters a given type uses. The analyses are defined in
`src/ir/analysis/*`. They are implemented as fixed-point algorithms, using the
`ir::analysis::MonotoneFramework` trait.

The final phase is generating Rust source text from the analyzed IR, and it is
defined in `src/codegen/*`. We use the `quote` crate, which provides the `quote!
{ ... }` macro for quasi-quoting Rust forms. Some options that affect the
generated Rust code are implemented using the [`syn`](https://docs.rs/syn) crate.

### Implementing new options using `syn`

If a new option can be implemented using the `syn` crate it should be added to
the `codegen::postprocessing` module by following these steps:

- Introduce a new field to `BindgenOptions` for the option.
- Write a free function inside `codegen::postprocessing` implementing the
  option. This function with the same name of the `BindgenOptions` field.
- Add a new value to the `codegen::postprocessing::PASSES` for the option using
  the `pass!` macro.

## Pull Requests and Code Reviews

Ensure that each commit stands alone, and passes tests. This enables better `git
bisect`ing when needed. If your commits do not stand on their own, then rebase
them on top of the latest main and squash them into a single commit.

All pull requests undergo code review before merging. To request review, comment
`r? @github_username_of_reviewer`. They we will respond with `r+` to approve the
pull request, or may leave feedback and request changes to the pull request. Any
changes should be squashed into the original commit.

Unsure who to ask for review? Ask any of:

* `@emilio`
* `@pvdrz`

More resources:

* [Servo's GitHub Workflow](https://github.com/servo/servo/wiki/Github-workflow)
* [Beginner's Guide to Rebasing and Squashing](https://github.com/servo/servo/wiki/Beginner's-guide-to-rebasing-and-squashing)

## Generating Graphviz Dot Files

We can generate [Graphviz](http://graphviz.org/pdf/dotguide.pdf) dot files from
our internal representation of a C/C++ input header, and then you can create a
PNG or PDF from it with Graphviz's `dot` program. This is very useful when
debugging bindgen!

First, make sure you have Graphviz and `dot` installed:

```
$ brew install graphviz         # OS X
$ sudo dnf install graphviz     # Fedora
$ # Etc...
```

Then, use the `--emit-ir-graphviz` flag to generate a `dot` file from our IR:

```
$ cargo run -- example.hpp --emit-ir-graphviz output.dot
```

Finally, convert the `dot` file to an image:

```
$ dot -Tpng output.dot -o output.png
```

The final result will look something like this:

[![An example graphviz rendering of our IR](./example-graphviz-ir.png)](./example-graphviz-ir.png)

## Debug Logging

To help debug what `bindgen` is doing, you can define the environment variable
`RUST_LOG=bindgen` to get a bunch of debugging log spew.

```
$ RUST_LOG=bindgen ./target/debug/bindgen [flags...] ~/path/to/some/header.h
```

This logging can also be used when debugging failing tests:

```
$ RUST_LOG=bindgen cargo test
```

## Using `creduce` to Minimize Test Cases

If you find a test case that triggers an unexpected panic in `bindgen`, causes
`bindgen` to emit bindings that won't compile, define structs with the wrong
size/alignment, or results in any other kind of incorrectness, then using
`creduce` can help reduce the test case to a minimal one that still exhibits
that same bad behavior.

***Reduced test cases are SUPER helpful when filing bug reports!***

### Getting `creduce`

Often, you can install `creduce` from your OS's package manager:

```
$ sudo apt install creduce
$ brew install creduce
$ # Etc...
```

Otherwise, follow [these instructions](https://github.com/csmith-project/creduce/blob/main/INSTALL.md) for building and/or installing `creduce`.

Running `creduce` requires two things:

1. Your isolated test case, and

2. A script to act as a predicate script describing whether the behavior you're
   trying to isolate occurred.

With those two things in hand, running `creduce` looks like this:

    $ creduce ./predicate.sh ./isolated-test-case.h

### Isolating Your Test Case

If you're using `bindgen` as a command line tool, pass
`--dump-preprocessed-input` flag.

If you're using `bindgen` as a Rust library, invoke the
`bindgen::Builder::dump_preprocessed_input` method where you call
`bindgen::Builder::generate`.

Afterwards, there should be a `__bindgen.i` or `__bindgen.ii` file containing
the combined and preprocessed input headers, which is usable as an isolated,
standalone test case.

### Writing a Predicate Script

Writing a `predicate.sh` script for a `bindgen` test case is straightforward. We
already have a general purpose predicate script that you can use, you just have
to wrap and configure it.

```bash
#!/usr/bin/env bash

# Exit the script with a nonzero exit code if:
# * any individual command finishes with a nonzero exit code, or
# * we access any undefined variable.
set -eu

# Invoke the general purpose predicate script that comes in the
# `bindgen` repository.
#
# You'll need to replace `--whatever-flags` with things that are specific to the
# incorrectness you're trying to pin down. See below for details.
path/to/rust-bindgen/csmith-fuzzing/predicate.py \
    --whatever-flags \
    ./isolated-test-case.h
```

When hunting down a particular panic emanating from inside `bindgen`, you can
invoke `predicate.py` like this:

```bash
path/to/rust-bindgen/csmith-fuzzing/predicate.py \
    --expect-bindgen-fail \
    --bindgen-grep "thread main panicked at '<insert panic message here>'" \
    ./isolated-test-case.h
```

Alternatively, when hunting down a bad `#[derive(Eq)]` that is causing `rustc`
to fail to compile `bindgen`'s emitted bindings, you can invoke `predicate.py`
like this:

```bash
path/to/rust-bindgen/csmith-fuzzing/predicate.py \
    --bindings-grep NameOfTheStructThatIsErroneouslyDerivingEq \
    --expect-compile-fail \
    --rustc-grep 'error[E0277]: the trait bound `f64: std::cmp::Eq` is not satisfied' \
    ./isolated-test-case.h
```

Or, when minimizing a failing layout test in the compiled bindings, you can
invoke `predicate.py` like this:

```bash
path/to/rust-bindgen/csmith-fuzzing/predicate.py \
    --bindings-grep MyStruct \
    --expect-layout-tests-fail \
    --layout-tests-grep "thread 'bindgen_test_layout_MyStruct' panicked" \
    ./isolated-test-case.h
```

For details on all the flags that you can pass to `predicate.py`, run:

```
$ path/to/rust-bindgen/csmith-fuzzing/predicate.py --help
```

And you can always write your own, arbitrary predicate script if you prefer.
(Although, maybe we should add extra functionality to `predicate.py` -- file an
issue if you think so!)

`creduce` is *really* helpful and can cut hundreds of thousands of lines of test
case down to 5 lines.

Happy bug hunting and test case reducing!

[More information on using `creduce`.](https://embed.cs.utah.edu/creduce/using/)

## Cutting a new bindgen release

To cut a release, the following needs to happen:

### Updating the changelog

Update the CHANGELOG.md file with the changes from the last release. Something
like the following is a useful way to check what has landed:

 ```
 $ git log --oneline v0.62.0..HEAD
 ```

Also worth checking the [next-release
tag](https://github.com/rust-lang/rust-bindgen/pulls?q=is%3Apr+label%3Anext-release).
It is very important that you do not rename the `Unreleased` section of the
changelog as this will be done automatically using `cargo release` on a further
step.

### Merge to `main`

For regular releases, the changes above should end up in `main` before
publishing. For dot-releases of an old version (e.g., cherry-picking an
important fix) you can skip this.

### Tag and publish

Once you're in `main`. Remember to install `doctoc` by running:
```
npm install doctoc
```

And then run:
```
cargo release [patch|minor] --no-publish --execute
```

This does the following:

- Bump the version.
- Turn the `Unreleased` section of the changelog into the section for the version being published.
- Update the table of contents of the changelog using `doctoc`.
- Tag (`git tag`) the HEAD commit
- Publish (`cargo publish`) bindgen and bindgen-cli
- Push (`git push`) to GitHub

The `patch` and `minor` refer to semver concepts:

- `patch` would bump __v0.68.1__ to __v0.68.2__
- `feature` would bump __v0.68.2__ to __v0.69.0__

### Create a new release on Github

The release is automated with the help of `.github/workflows/release.yml`,
and will only be created...

- when a Git tag is pushed
- when all tests succeed

While the tests are still running,
a draft GitHub release will be created,
to avoid notifying watchers of the repo should a CI step fail.

If everything succeeds,
bindgen cli installers for Linux/MacOS and Windows will be created,
as well as tarballs.
See `[workspace.metadata.dist]` section in Cargo.toml for the configuration.

To update the release configuration,
when a new cargo-dist is available:

```
cargo dist init # from "cargo install cargo-dist"
cargo dist generate-ci # to update .github/workflows/release.yml
```

### What to do if a Github release fails

If the release process failed after you run `cargo release` you can manually
delete the tag and release from Github. Also remember to delete the tag locally
by running `git tag -d`. Once all the extra changes are in the `main` branch
you can trigger a release by creating a new tag using `git tag` and push it
using `git push --tag`.

### Create a new crates.io release

Go to [the Publish
workflow](https://github.com/rust-lang/rust-bindgen/actions/workflows/publish.yml)
and run a new workflow using the "Run Workflow" button.

Remember that crates.io releases cannot be deleted!

[prettyplease]: https://github.com/dtolnay/prettyplease
