# Annotating types with `#[must-use]`

`bindgen` can be instructed to annotate certain types with 
[`#[must_use]`](https://doc.rust-lang.org/reference/attributes/diagnostics.html#the-must_use-attribute).

Some libraries have a common error type, returned by lots of their functions, 
which needs to be checked after every call. In these cases it's useful to add `#[must_use]` to this type, so the Rust
compiler emits a warning when the check is missing.
### Library

* [`bindgen::Builder::must_use_type`](https://docs.rs/bindgen/latest/bindgen/struct.Builder.html#method.must_use_type)

### Command Line

* `--must-use-type <regex>`

### Annotations

```c
/** <div rustbindgen mustusetype></div> */
struct ErrorType {
    // ...
};

...
```

