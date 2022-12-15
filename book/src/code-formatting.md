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
