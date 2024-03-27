# Code Formatting

`bindgen` uses `rustfmt` to format the emitted bindings. This section describes
how to adjust the `rustfmt` behavior when being used from `bindgen`.

## Passing a `rustfmt.toml` configuration file

`rustfmt` should automatically use any `rustfmt.toml` file that is present in
the directory from where `bindgen` will be run. If you want to use a
configuration file that has a different name or that is in a different
directory you can use the `--rustfmt-configuration-file` flag or the
[`Builder::rustfmt_configuration_file`](https://docs.rs/bindgen/latest/bindgen/struct.Builder.html#method.rustfmt_configuration_file)
method.

## Using a nightly release of `rustfmt`

If the `rustfmt` command does not correspond to a nightly release of `rustfmt`
but you have `rustup` available, you can use `nightly` by following these
steps:

### When using `bindgen` as a CLI application

Use `rustup run` to run `bindgen`:

```bash
$ rustup run nightly bindgen [ARGS]
```

### When using `bindgen` as a library

Take the output of the following command:
```bash
$ rustup which rustfmt --toolchain=nightly
```
and pass it to
[`Builder::with_rustfmt`](https://docs.rs/bindgen/latest/bindgen/struct.Builder.html#method.with_rustfmt):

```rust,ignore
use bindgen::Builder;
use std::process::Command;

fn main() {
    let output = Command::new("rustup")
        .args(["which", "rustfmt", "--toolchain", "nightly"])
        .output()
        .expect("Could not spawn `rustup` command");

    assert!(
        output.status.success(),
        "Unsuccessful status code when running `rustup`: {:?}",
        output
    );

    let rustfmt_path =
        String::from_utf8(output.stdout).expect("The `rustfmt` path is not valid `utf-8`");

    let bindings = Builder::default()
        .header("path/to/input.h")
        .with_rustfmt(rustfmt_path)
        .generate()
        .expect("Could not generate bindings");

    bindings
        .write_to_file("path/to/output.rs")
        .expect("Could not write bindings");
}
```

These two methods also apply to any other toolchain available in your system.

## Using `prettyplease`

The [`prettyplease`](https://github.com/dtolnay/prettyplease) crate is a
minimal formatter for generated code. To format bindings using `prettyplease`
you have to invoke `bindgen` with either the `--formatter=prettyplease` flag or
the `bindgen::Builder::formatter(bindgen::Formatter::Prettyplease)`. One of
its advantages is that `prettyplease` can be used in minimal environments where
the Rust toolchain is not installed.

## How can I normalize `#[doc]` attributes?

`bindgen` emits all the documentation using `#[doc]` attributes by default. If
you want to use the more user-friendly `///` syntax, you have two options:

### Use `rustfmt`

`rustfmt` can be configured to normalize documentation. To do so, you have to
create a `rustfmt.toml` file with the following contents:

```toml
normalize_doc_attributes = true
```

Then, you have set up `bindgen` so it passes this file to `rustfmt`. Given that
the `normalize_doc_attributes` option is
[unstable](https://github.com/rust-lang/rustfmt/issues/3351), you also have to
set up bindgen to use a `nightly` release of `rustfmt`.


### Use `prettyplease`

`prettyplease` normalizes documentation without any additional configuration.
Then you just have to tell `bindgen` to use `prettyplease` as the code
formatter.
