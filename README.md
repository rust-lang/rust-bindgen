# rust-bindgen

[![][crates-version-shield]](https://crates.io/crates/bindgen)
[![][crates-downloads-shield]](https://crates.io/crates/bindgen)
[![][crates-license-shield]](https://github.com/crabtw/rust-bindgen/blob/master/LICENSE.txt)
[![][travis-status-shield]](https://travis-ci.org/crabtw/rust-bindgen)

A native binding generator for the Rust language.

*rust-bindgen* was originally ported from [clay's bindgen].

[Documentation](https://yamakaky.github.io/rust-bindgen/)

## Requirements

* Clang >= 3.5

## Installing

    $ cargo install bindgen

Bindgen will be dynamically linked to your default clang version. See
[clang-sys](https://github.com/KyleMayes/clang-sys/blob/master/README.md) if you
want to use an other version or do a static link build. The clang-sys feature
`static` can be activated via the `bindgen` feature `clang_sys/static`.

## Usage

### Command Line

    $ bindgen <header> [<bindgen options>] [-- <clang options>]

See `--help` for a list of the supported options.

### Plugin

```rust
    bindgen!(header, options...)
```

The use of this plugin requires the use of a nightly compiler.

Options:

| Option Name         | Type | Default |
| ------------------- | ---- | ------- |
| link                | str  |         |
| match               | str  |         |
| builtins            | bool | true    |
| allow_unknown_types | bool | false   |
| clang_args          | str  |         |

## Examples

### Generate a Lua binding with the CLI

    bindgen --link lua --builtins /usr/include/lua.h -o lua.rs

### Generate a Lua binding with the plugin

`Cargo.toml`

    [dependencies]
    bindgen = "*"

`main.rs`

    #![feature(plugin)]
    #![plugin(bindgen)]

    mod lua_bindings {
        bindgen!("/usr/include/lua.h", link="lua", builtins=true)
    }

### Using a build script to generate bindings at compile time

Due to a known issuewith `include!` https://github.com/rust-lang/rfcs/issues/752 when generating
bindings in a build script and importing them with `include!` you'll want to wrap the bindings
in a module before writing them to a file to avoid triggering the issue with top-level
attributes in `include!`. Some more discussion about this issue can be found here
https://github.com/Yamakaky/rust-bindgen/issues/359 .

`Cargo.toml`
```rust
[package]
...
name = "bindgen_ex"
build = "build.rs"

[build-dependencies]
bindgen = "0.19.0"
```

`build.rs`
```rust
extern crate bindgen;

use std::io::prelude::*;
use std::fs::File;

fn main(){
    let mut bindings = bindgen::Builder::new("my_lib.h");
    bindings.link("my_lib", bindgen::LinkType::Static);
    // Generate the bindings to a string so we can wrap them
    // instead of going through the `write_to_file` API.
    let generated_bindings = bindings.generate().expect("Failed to generate bindings");
    // Now open the file we'll write the generated bindings too
    let mut file = File::create("my_lib.rs").expect("Failed to open file");
    // Wrap the bindings in a `pub mod` before writing bindgen's output
    file.write(format!("pub mod {} {{\n", "my_lib").as_bytes()).unwrap();
    file.write(generated_bindings.as_bytes()).unwrap();
    file.write(b"}").unwrap(); 
}
```

`main.rs`
```rust
include!("my_lib.rs");

fn main() {
    my_lib::example_function();
}
```

[crates-version-shield]: https://img.shields.io/crates/v/bindgen.svg?style=flat-square
[crates-downloads-shield]: https://img.shields.io/crates/d/bindgen.svg?style=flat-square
[crates-license-shield]: https://img.shields.io/crates/l/bindgen.svg?style=flat-square
[travis-status-shield]: https://img.shields.io/travis/crabtw/rust-bindgen/master.svg?label=travis&style=flat-square

[clay's bindgen]: https://github.com/jckarter/clay/blob/master/tools/bindgen.clay
[issue 89]: https://github.com/yamakaky/rust-bindgen/issues/89
