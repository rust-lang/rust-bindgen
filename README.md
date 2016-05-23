rust-bindgen
============
[![][crates-version-shield]](https://crates.io/crates/bindgen)
[![][crates-downloads-shield]](https://crates.io/crates/bindgen)
[![][crates-license-shield]](https://github.com/crabtw/rust-bindgen/blob/master/LICENSE)
[![Build Status](https://travis-ci.org/crabtw/rust-bindgen.svg?branch=master)](https://travis-ci.org/crabtw/rust-bindgen)

A binding generator for the rust language.
It is ported from [clay's bindgen][].

Requirements
------------

* clang 3.4 and up

Note: The libclang.so has to be statically linked with LLVM or you will
encounter [issue 89][]. You can also use LD_PRELOAD=/path/to/libclang.so to
workaround the problem.

Building
--------

    $ cargo build

Note: This links with Apple's version of libclang on OS X by default. This can be changed by setting the LIBCLANG_PATH environment variable.

If you are running the command line tool you will also need to append this
path to your DYLD_LIBRARY_PATH environment variable, which you might already have set if you have installed the Rust compiler outside of standard /usr/local path.

The default path on OS X is:

    /Applications/Xcode.app/Contents/Developer/Toolchains/XcodeDefault.xctoolchain/usr/lib/

Or if you only have Xcode Command Line Tools installed:

    export DYLD_LIBRARY_PATH=/Library/Developer/CommandLineTools/usr/lib

Command Line Usage
------------------

```
Usage: ./bindgen [options] input.h
Options:
    -h or --help               Display help message
    -l <name> or -l<name>      Link to a dynamic library, can be provided
                               multiple times
    -static-link <name>        Link to a static library
    -framework-link <name>     Link to a framework
    -o <output.rs>             Write bindings to <output.rs> (default stdout)
    -match <name>              Only output bindings for definitions from files
                               whose name contains <name>
                               If multiple -match options are provided, files
                               matching any rule are bound to
    -builtins                  Output bindings for builtin definitions
                               (for example __builtin_va_list)
    -allow-unknown-types       Don't fail if we encounter types we do not support,
                               instead treat them as void
    -emit-clang-ast            Output the ast (for debugging purposes)
    -override-enum-type <type> Override enum type, type name could be
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

    Options other than stated above are passed to clang
```

Macro Usage
-----------

```
Usage: bindgen!([headers], [named options])
Options:

    Option Name          Type              Default
    ----------------------------------------------
    link                 multiple strings
    link_static          multiple strings
    link_framework       multiple strings
    match                multiple strings
    emit_builtins        bool              true
    allow_unknown_types  bool              false
    clang_args           string
```
See "Command Line Usage" section for option descriptions

Examples
--------

###Generate MySQL client bindings

    bindgen -l mysql -match mysql.h -o mysql.rs /usr/include/mysql/mysql.h

*or*

    echo '#include <mysql.h>' > gen.h
    bindgen `mysql_config --cflags` -l mysql -match mysql.h -o mysql.rs gen.h

*or*

Cargo.toml

    [dependencies.bindgen]
    git = "https://github.com/crabtw/rust-bindgen.git"

main.rs

    #![feature(phase)]
    #[phase(plugin)] extern crate bindgen;

    #[allow(dead_code, uppercase_variables, non_camel_case_types)]
    mod mysql_bindings {
        bindgen!("/usr/include/mysql/mysql.h", match="mysql.h", link="mysql")
    }

TODO
----

* bitfield accessors

[clay's bindgen]: https://github.com/jckarter/clay/blob/master/tools/bindgen.clay
[crates-version-shield]: https://img.shields.io/crates/v/bindgen.svg?style=flat-square
[crates-downloads-shield]: https://img.shields.io/crates/d/bindgen.svg?style=flat-square
[crates-license-shield]: https://img.shields.io/crates/l/bindgen.svg?style=flat-square
[travis-status-shield]: https://img.shields.io/travis/crabtw/rust-bindgen.svg?label=travis&style=flat-square
[issue 89]: https://github.com/crabtw/rust-bindgen/issues/89
