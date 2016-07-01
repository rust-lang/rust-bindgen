rust-bindgen
============
[![][crates-version-shield]](https://crates.io/crates/bindgen)
[![][crates-downloads-shield]](https://crates.io/crates/bindgen)
[![][crates-license-shield]](https://github.com/crabtw/rust-bindgen/blob/master/LICENSE)
[![][travis-status-shield]](https://travis-ci.org/crabtw/rust-bindgen/)

A binding generator for the rust language.
This is a fork designed to work on Spidermonkey headers.
It is ported from [clay's bindgen][].

Requirements
------------

* clang 3.7 with patches https://github.com/michaelwu/clang/tree/release_37_smhacks or clang 3.8+

This bindgen fork requires a patched clang or clang 3.8+ to work properly. This is one way to build a patched clang:
```
git clone https://github.com/llvm-mirror/llvm
cd llvm
git checkout release_37
cd tools
git clone https://github.com/llvm-mirror/clang
cd clang
git remote add mwu https://github.com/michaelwu/clang
git fetch mwu
git checkout release_37_smhacks
cd ../.. # llvm root dir
mkdir build
cd build
../configure --enable-optimized
make
```

Then before building, make sure to export the path to this copy of clang:

    export LIBCLANG_PATH=~/llvm/build/Release+Asserts/lib

This path also needs to be set in LD_LIBRARY_PATH (Linux) or DYLD_LIBRARY_PATH (OSX) when running bindgen.

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

C++ Usage
---------
This fork of rust-bindgen can handle a number of C++ features. Because it currently uses a fork of clang though, it may require adding extra arguments to find certain headers. On OpenSUSE 13.2, these additional include pathes can be used:

    -isystem /usr/lib64/gcc/x86_64-suse-linux/4.8/include -isystem /usr/lib64/gcc/x86_64-suse-linux/4.8/include-fixed

On OSX, this include path seems to work:

    -isystem /Applications/Xcode.app/Contents/Developer/Toolchains/XcodeDefault.xctoolchain/usr/include/c++/v1

When passing in header files, the file will automatically be treated as C++ if it ends in ``.hpp``. If it doesn't, ``-x c++`` can be used to force C++ mode.

Annotations
-----------
The translation of classes, structs, enums, and typedefs can be adjusted using annotations. Annotations are specifically formatted html tags inside doxygen style comments. The opaque annotation instructs bindgen to ignore all fields defined in a struct/class.

    /// <div rustbindgen opaque></div>

The hide annotation instructs bindgen to ignore the struct/class/field/enum completely.

    /// <div rustbindgen hide></div>

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
