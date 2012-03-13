A binding generator for the rust language.
It is ported from clay's bindgen.

=== Requirements ===

* clang 3.1svn
* patch for x86-64 ABI
    * https://github.com/mozilla/rust/pull/1970

=== TODO ===

* array type
    * current implemented by tuple
* union type
* recursive type
* global variable
* variadic arguments
