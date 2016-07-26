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

[crates-version-shield]: https://img.shields.io/crates/v/bindgen.svg?style=flat-square
[crates-downloads-shield]: https://img.shields.io/crates/d/bindgen.svg?style=flat-square
[crates-license-shield]: https://img.shields.io/crates/l/bindgen.svg?style=flat-square
[travis-status-shield]: https://img.shields.io/travis/crabtw/rust-bindgen/master.svg?label=travis&style=flat-square

[clay's bindgen]: https://github.com/jckarter/clay/blob/master/tools/bindgen.clay
[issue 89]: https://github.com/yamakaky/rust-bindgen/issues/89
