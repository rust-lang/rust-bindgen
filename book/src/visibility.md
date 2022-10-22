# Making fields private

Fields can be made private for various reasons. You may wish to enforce some invariant on the fields of a structure, which cannot be achieved if the field is public and can be set by any code. For example, you may wish to ensure that a pointer always points to something appropriate.

### Annotation

```c
struct OneFieldPrivate {
    /** Null-terminated, static string. <div rustbindgen private> */
    const char *s;
    bool b;
};

/** <div rustbindgen private> */
struct MostFieldsPrivate {
    int a;
    bool b;
    /** <div rustbindgen private="false"></div> */
    char c;
};
```

Then in Rust:

```rust
impl OneFieldPrivate {
    pub fn new(s: &'static core::ffi::CStr, b: bool) -> Self {
        OneFieldPrivate { s: s.as_ptr(), b }
    }
}
```
