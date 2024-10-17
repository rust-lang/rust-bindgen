# Create a `build.rs` File

We create a `build.rs` file in our crate's root. Cargo will pick up on the existence of this file, then compile and execute it before the rest of the crate is built.
This can be used to generate code at compile time.
And of course in our case, we will be generating Rust FFI
bindings to `bzip2` at compile time. The resulting bindings will be written to
`$OUT_DIR/bindings.rs` where `$OUT_DIR` is chosen by `cargo` and is something
like `./target/debug/build/bindgen-tutorial-bzip2-sys-afc7747d7eafd720/out/`.

Note that the associated shared object to `bz2` is `libbz2.so`. In general, a `lib<name>.so` should be referenced in the build file by `<name>`.

```rust,ignore
use std::env;
use std::path::PathBuf;

fn main() {
    // Tell cargo to look for shared libraries in the specified directory
    println!("cargo:rustc-link-search=/path/to/lib");

    // Tell cargo to tell rustc to link the system bzip2
    // shared library.
    println!("cargo:rustc-link-lib=bz2");

    // The bindgen::Builder is the main entry point
    // to bindgen, and lets you build up options for
    // the resulting bindings.
    let bindings = bindgen::Builder::default()
        // The input header we would like to generate
        // bindings for.
        .header("wrapper.h")
        // Tell cargo to invalidate the built crate whenever any of the
        // included header files changed.
        .parse_callbacks(Box::new(bindgen::CargoCallbacks))
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

Now, when we run `cargo build`, our bindings to `bzip2` are generated on the
fly!

[There's more info about `build.rs` files in the Cargo documentation.][build-rs]

[build-rs]: https://doc.rust-lang.org/cargo/reference/build-scripts.html
