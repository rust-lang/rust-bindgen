# rust-bindgen

[![][crates-version-shield]](https://crates.io/crates/bindgen)
[![][crates-downloads-shield]](https://crates.io/crates/bindgen)
[![][crates-license-shield]](https://github.com/crabtw/rust-bindgen/blob/master/LICENSE)
[![Build Status][travis-status-shield]](https://travis-ci.org/crabtw/rust-bindgen)

A native binding generator for the Rust language.

*rust-bindgen* was originally ported from [clay's bindgen].

## Requirements

* Clang >= 3.4

*Note:* `libclang.so` has to be statically linked with LLVM or you will encounter [issue 89]. You can also use `LD_PRELOAD=/path/to/libclang.so` to workaround the problem.

## Building

    $ cargo build

Note: this links with Apple's version of `libclang` on OS X, by default. This can be changed by setting the `LIBCLANG_PATH` environment variable.

If you are running the command line tool you will also need to append the following path to the `DYLD_LIBRARY_PATH` environment variable. You might already have done this if you have installed the Rust compiler outside of standard `/usr/local` directory.

    /Applications/Xcode.app/Contents/Developer/Toolchains/XcodeDefault.xctoolchain/usr/lib/

Or, if you only have Xcode Command Line Tools installed:

    /Library/Developer/CommandLineTools/usr/lib

## Usage

### Command Line

```
Usage: bindgen [OPTIONS] HEADERS...

Options:
    -h, --help                  Display help message
    -l [KIND=]NAME              Link to the specified library NAME. The optional KIND can be one of,
                                static, dylib, or framework. If omitted, dylib is assumed.
    -o FILENAME                 Write generated bindings to FILENAME (default is stdout)
    -match NAME                 Only output bindings for definitions from files whose names contain
                                NAME. Can be used multiples times to include files matching any of
                                the names.
    -builtins                   Output bindings for builtin definitions (for example,
                                `__builtin_va_list`)
    -allow-unknown-types        Do not fail if unknown types are encountered; instead treat them as
                                `void`
    -emit-clang-ast             Output the AST (for debugging purposes)
    -override-enum-type TYPE    Override the integer type for enums, where TYPE is one of:
                                  uchar
                                  schar
                                  ushort
                                  sshort
                                  uint
                                  sint
                                  ulong
                                  slong
                                  ulonglong
                                  slonglong
    
    Options other than the above are passed to Clang.
```

### Macro

    bindgen!(header..., options...)

Options:

| Option Name         | Type | Default |
| ------------------- | ---- | ------- |
| link                | str  |         |
| match               | str  |         |
| builtins            | bool | true    |
| allow_unknown_types | bool | false   |
| clang_args          | str  |         |

See the Usage · [Command Line](#command-line) section for descriptions of options.

### Examples

#### Generate MySQL client bindings

    bindgen -l mysql -match mysql.h -o mysql.rs /usr/include/mysql/mysql.h

*or*

    echo '#include <mysql.h>' > gen.h
    bindgen `mysql_config --cflags` -l mysql -match mysql.h -o mysql.rs gen.h

*or*

`Cargo.toml`

    [dependencies]
    bindgen = ">= 0"

`main.rs`

    #![feature(phase)]
    #[phase(plugin)] extern crate bindgen;
    
    #[allow(dead_code, uppercase_variables, non_camel_case_types)]
    mod mysql_bindings {
        bindgen!("/usr/include/mysql/mysql.h", match="mysql.h", link="mysql")
    }

To Do
-----

* Bit-field accessors

[crates-version-shield]: https://img.shields.io/crates/v/bindgen.svg?style=flat-square
[crates-downloads-shield]: https://img.shields.io/crates/d/bindgen.svg?style=flat-square
[crates-license-shield]: https://img.shields.io/crates/l/bindgen.svg?style=flat-square
[travis-status-shield]: https://img.shields.io/travis/crabtw/rust-bindgen/master.svg?label=travis&style=flat-square

[clay's bindgen]: https://github.com/jckarter/clay/blob/master/tools/bindgen.clay
[issue 89]: https://github.com/crabtw/rust-bindgen/issues/89
