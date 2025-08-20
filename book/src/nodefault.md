# Preventing the Derivation of `Default`

By default, `Default` is not derived.

`bindgen` will attempt to derive/impl the `Default` traits on a best-effort basis.
Sometimes, we need customize the implementation of `Default` for certain types.
In these cases, the `nodefault` annotation can be used to prevent bindgen from
automatically deriving the `Default` trait for a type.

### Library

* [`bindgen::Builder::derive_default`](https://docs.rs/bindgen/latest/bindgen/struct.Builder.html#method.derive_default)
* [`bindgen::Builder::no_default`](https://docs.rs/bindgen/latest/bindgen/struct.Builder.html#method.no_default)

### Command Line

* `--with-derive-default`
* `--no-default <regex>`

### Annotations

```c
/**
 * We need to specify some preset values as the Default of Header.
 *
 * for example:
 *
 * <div rustbindgen nodefault></div>
 */
struct Header {
    unsigned int magic;
    unsigned char data[252];
};

...
```

### Customize Implements

```rust,ignore
// Include the generated bindings.
include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

impl Default for Header {
    fn default() -> Self {
        Self {
            magic: 0x10203040u32,
            data: [0; 252usize],
        }
    }
}
```
