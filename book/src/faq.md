# Frequently Asked Questions

<!-- START doctoc generated TOC please keep comment here to allow auto update -->
<!-- DON'T EDIT THIS SECTION, INSTEAD RE-RUN doctoc TO UPDATE -->

- [Why isn't `bindgen` generating methods for this allowlisted class?](#why-isnt-bindgen-generating-methods-for-this-allowlisted-class)
- [Why isn't `bindgen` generating bindings to inline functions?](#why-isnt-bindgen-generating-bindings-to-inline-functions)
- [Does `bindgen` support the C++ Standard Template Library (STL)?](#does-bindgen-support-the-c-standard-template-library-stl)
- [How to deal with bindgen generated padding fields?](#how-to-deal-with-bindgen-generated-padding-fields)

<!-- END doctoc generated TOC please keep comment here to allow auto update -->

### Why isn't `bindgen` generating methods for this allowlisted class?

Are the methods `inline` methods, or defined inline in the class? For example:

```c++
class Dooder {
  public:
    // Function defined inline in the class.
    int example_one() { return 1; }

    // `inline` function whose definition is supplied later in the header, or in
    // another header.
    inline bool example_two();
};

inline bool Dooder::example_two() {
    return true;
}
```

If so, see
["Why isn't `bindgen` generating bindings to inline functions?"](#why-isnt-bindgen-generating-bindings-to-inline-functions)

If not, consider filing an issue!

### Why isn't `bindgen` generating bindings to inline functions?

These functions don't typically end up in object files or shared libraries with
symbols that we can reliably link to, since they are instead inlined into each
of their call sites. Therefore, we don't generate bindings to them, since that
creates linking errors.

However, if you are compiling the C/C++ yourself (rather than using a system
shared library, for example), then you can pass `-fkeep-inline-functions` or
`-fno-inline-functions` to `gcc` or `clang`, and invoke `bindgen` with either
the `bindgen::Builder::generate_inline_functions` method or the
`--generate-inline-functions` flag.

Note that these functions and methods are usually marked inline for a reason:
they tend to be hot. The above workaround makes them an out-of-line call, which
might not provide acceptable performance.

### Does `bindgen` support the C++ Standard Template Library (STL)?

Sort of. A little. Depends what you mean by "support".

Most functions, methods, constructors, and destructors are inline in the
STL. That ties our hands when it comes to linking: ["Why isn't `bindgen` generating bindings to inline functions?"](#why-isnt-bindgen-generating-bindings-to-inline-functions)

As far as generating opaque blobs of bytes with the correct size and alignment,
`bindgen` can do pretty well. This is typically enough to let you use types that
transitively contain STL things. We generally recommend marking `std::.*` as
opaque, and then allowlisting only the specific things you need from the library
you're binding to that is pulling in STL headers.

### How to deal with bindgen generated padding fields?

Depending the architecture, toolchain versions and source struct, it is
possible that bindgen will generate padding fields named `__bindgen_padding_N`. 
As these fields might be present when compiling for one architecture but not
for an other, you should not initialize these fields manually when initializing
the struct. Instead, use the `Default` trait. You can either enable this when
constructing the `Builder` using the `derive_default` method, or you can
implement this per struct using:

```rust,ignore
impl Default for SRC_DATA {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}
```

This makes it possible to initialize `SRC_DATA` by:

```rust,ignore
SRC_DATA {
    field_a: "foo",
    field_b: "bar",
    ..Default::default()
}
```

In the case bindgen generates a padding field, then this field will
be automatically initialized by `..Default::default()`.
