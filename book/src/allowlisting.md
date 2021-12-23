# Allowlisting

Allowlisting allows us to be precise about which type, function, and global
variable definitions `bindgen` generates bindings for. By default, if we don't
specify any allowlisting rules, everything is considered allowlisted. This may
not be desirable because of either

* the generated bindings contain a lot of extra definitions we don't plan on using, or
* the header file contains C++ features for which Rust does not have a
  corresponding form (such as partial template specialization), and we would
  like to avoid these definitions

If we specify allowlisting rules, then `bindgen` will only generate bindings to
types, functions, and global variables that match the allowlisting rules, or are
transitively used by a definition that matches them.

### Library

* [`bindgen::Builder::allowlist_type`](https://docs.rs/bindgen/latest/bindgen/struct.Builder.html#method.allowlist_type)
* [`bindgen::Builder::allowlist_function`](https://docs.rs/bindgen/latest/bindgen/struct.Builder.html#method.allowlist_function)
* [`bindgen::Builder::allowlist_var`](https://docs.rs/bindgen/latest/bindgen/struct.Builder.html#method.allowlist_var)

### Command Line

* `--allowlist-type <type>`
* `--allowlist-function <function>`
* `--allowlist-var <var>`

### Annotations

None.
