A binding generator for the rust language.
It is ported from clay's bindgen[1].

[1] https://github.com/jckarter/clay/blob/master/tools/bindgen.clay

Requirements
------------

* clang 3.1

Usage
-----

Usage: ./bindgen [options] input.h
Options:
    -h or --help    Display help message
    -l <name>       Link name of the library
    -o <output.rs>  Write bindings to <output.rs> (default stdout)
    -match <name>   Only output bindings for definitions from files
                    whose name contains <name>
                    If multiple -match options are provided, files
                    matching any rule are bound to.

    Options other than stated above are passed to clang.

Example
-------

Generate MySQL client bindings

    bindgen -l mysql -match mysql.h -o mysql.rs /usr/local/include/mysql/mysql.h

or

    echo '#include <mysql.h>' > gen.h
    bindgen `mysql_config --cflags` -l mysql -match mysql.h -o mysql.rs gen.h

TODO
----

* bit field
* union type
* global variable
* variadic function
