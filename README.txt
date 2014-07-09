A binding generator for the rust language.
It is ported from clay's bindgen[1].

[1] https://github.com/jckarter/clay/blob/master/tools/bindgen.clay

Requirements
------------

* clang 3.4 and up

Note: The libclang.so has to be statically linked with LLVM
      or you will encounter https://github.com/crabtw/rust-bindgen/issues/89

Building
--------

    $ rustc lib.rs
    $ rustc -L . bindgen.rs

Note: If you want to use Apple's version of libclang on OS X, you will need
to add this to both commands:

  -C link-args="-L/Applications/Xcode.app/Contents/Developer/Toolchains/XcodeDefault.xctoolchain/usr/lib/"

You will also need to append this path to your DYLD_LIBRARY_PATH environment
variable, which you might already have set if you have installed the Rust
compiler outside of standard /usr/local path.

Command Line Usage
------------------

Usage: ./bindgen [options] input.h
Options:
    -h or --help           Display help message
    -l <name> or -l<name>  Link to a dynamic library, can be provided
                           multiple times
    -static-link <name>    Link to a static library
    -framework-link <name> Link to a framework
    -o <output.rs>         Write bindings to <output.rs> (default stdout)
    -match <name>          Only output bindings for definitions from files
                           whose name contains <name>
                           If multiple -match options are provided, files
                           matching any rule are bound to
    -builtins              Output bindings for builtin definitions
                           (for example __builtin_va_list)
    -abi <abi>             Indicate abi of extern functions (default C)
    -allow-bitfields       Don't fail if we encounter a bitfield
                           (note that bindgen does not support bitfields)
    -allow-unknown-types   Don't fail if we encounter types we do not support,
                           instead treat them as void
    -emit-clang-ast        Output the ast (for debugging purposes)

    Options other than stated above are passed to clang

Macro Usage
-----------

Usage: bindgen!([headers], [named options])
Options:

    Option Name          Type              Default
    ----------------------------------------------
    link                 multiple strings
    link_static          multiple strings
    link_framework       multiple strings
    match                multiple strings
    emit_builtins        bool              true
    abi                  string            "C"
    allow_bitfields      bool              false
    allow_unknown_types  bool              false
    clang_args           string

    See "Command Line Usage" section for option descriptions

Examples
--------

Generate MySQL client bindings

    bindgen -l mysql -match mysql.h -o mysql.rs /usr/include/mysql/mysql.h

or

    echo '#include <mysql.h>' > gen.h
    bindgen `mysql_config --cflags` -l mysql -match mysql.h -o mysql.rs gen.h

or

    #![feature(phase)]
    #[phase(plugin)] extern crate bindgen;

    #[allow(dead_code, uppercase_variables, non_camel_case_types)]
    mod mysql_bindings {
        bindgen!("/usr/include/mysql/mysql.h", match="mysql.h", link="mysql")
    }

TODO
----

* bit field
