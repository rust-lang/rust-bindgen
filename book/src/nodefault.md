# Preventing the Derivation of `Default`

`bindgen` will attempt to derive/impl the `Default` traits on a best-effort basis.
Sometimes, we need customize the implement of `Default` for certain types,
In these cases, the `nodefault` annotation can be used to prevent bindgen 
to autoderive the `Default` traits for a type.

### Library

* [`bindgen::Builder::no_default`](https://docs.rs/bindgen/latest/bindgen/struct.Builder.html#method.no_default)

### Command Line

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
