# Finding Libraries with `pkg-config`

Many C libraries install their headers somewhere that `clang` does not search
by default. Some keep them in a versioned subdirectory such as
`/usr/include/libpng16`, and package managers like Homebrew install them under
their own prefix. In that case `bindgen` fails with an error like
`fatal error: 'png.h' file not found`.

Such libraries usually ship a [`pkg-config`][pkg-config] file describing where
their headers and binaries live. The [`pkg-config` crate][pkg-config-crate] can
query it from `build.rs`, and we can pass the include paths it finds to
`bindgen`.

First, add `pkg-config` to the `[build-dependencies]` section next to `bindgen`:

```toml
[build-dependencies]
bindgen = "0.73.2"
pkg-config = "0.3"
```

Our `wrapper.h` includes the library's header as usual:

```c
#include <png.h>
```

Then, in `build.rs`, we probe for the library and turn each include path into
a `-I` argument for `clang`:

```rust,ignore
use std::env;
use std::path::PathBuf;

fn main() {
    // Ask pkg-config for `libpng`. On success, this also prints the
    // `cargo:rustc-link-search` and `cargo:rustc-link-lib` lines needed to
    // link against the library.
    let library = pkg_config::probe_library("libpng")
        .expect("Unable to find libpng with pkg-config");

    let bindings = bindgen::Builder::default()
        // The input header we would like to generate
        // bindings for.
        .header("wrapper.h")
        // Tell clang where to find the headers included by `wrapper.h`.
        .clang_args(
            library
                .include_paths
                .iter()
                .map(|path| format!("-I{}", path.display())),
        )
        // Tell cargo to invalidate the built crate whenever any of the
        // included header files changed.
        .parse_callbacks(Box::new(bindgen::CargoCallbacks::new()))
        // Finish the builder and generate the bindings.
        .generate()
        // Unwrap the Result and panic on failure.
        .expect("Unable to generate bindings");

    // Write the bindings to the $OUT_DIR/bindings.rs file.
    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");
}
```

A few things to note:

- `bindgen` only generates the Rust declarations, it does not link anything.
  Linking is handled by `pkg_config::probe_library`, which prints the needed
  `cargo:rustc-link-*` lines, so there is no need to write them by hand as in
  the [tutorial](./tutorial-3.md).
- `bindgen::CargoCallbacks` prints a `cargo:rerun-if-changed` line for
  `wrapper.h` and for every header it includes, so the bindings are regenerated
  when the library's headers change. The `pkg-config` crate prints
  `cargo:rerun-if-env-changed` lines for the environment variables it reads,
  such as `PKG_CONFIG_PATH`.
- Some libraries also require preprocessor definitions. These are available in
  `library.defines` and can be passed to `clang` as `-D` arguments in the same
  way.
- Use [`pkg_config::Config`][pkg-config-config] instead of `probe_library` to
  require a minimum version, prefer static linking, and so on.

## Using `system-deps`

The [`system-deps` crate][system-deps] wraps `pkg-config` and lets us declare
the required libraries and versions in `Cargo.toml` instead:

```toml
[build-dependencies]
bindgen = "0.73.2"
system-deps = "9"

[package.metadata.system-deps]
libpng = "1.6"
```

In `build.rs`, `system_deps::Config::new().probe()` finds all the declared
libraries and prints the linker flags for them. The include paths are passed to
`bindgen` just like before:

```rust,ignore
let deps = system_deps::Config::new()
    .probe()
    .expect("Unable to find system dependencies");

let bindings = bindgen::Builder::default()
    .header("wrapper.h")
    .clang_args(
        deps.all_include_paths()
            .iter()
            .map(|path| format!("-I{}", path.display())),
    )
    .parse_callbacks(Box::new(bindgen::CargoCallbacks::new()))
    .generate()
    .expect("Unable to generate bindings");
```

[pkg-config]: https://www.freedesktop.org/wiki/Software/pkg-config/
[pkg-config-crate]: https://docs.rs/pkg-config
[pkg-config-config]: https://docs.rs/pkg-config/latest/pkg_config/struct.Config.html
[system-deps]: https://docs.rs/system-deps
