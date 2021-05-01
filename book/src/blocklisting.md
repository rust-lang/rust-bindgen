# Blocklisting

If you need to provide your own custom translation of some type (for example,
because you need to wrap one of its fields in an `UnsafeCell`), you can
explicitly blocklist
 generation of its definition. Uses of the blocklisted type
will still appear in other types' definitions. (If you don't want the type to
appear in the bindings at
all, [make it opaque](./opaque.md) instead of
blocklisting it.)

Blocklisted types are pessimistically assumed not to be able to `derive` any
traits, which can transitively affect other types' ability to `derive` traits or
not.

### Library

* [`bindgen::Builder::blocklist_type`](https://docs.rs/bindgen/latest/bindgen/struct.Builder.html#method.blocklist_type)

### Command Line

* `--blocklist-type <type>`

### Annotations

```cpp
/// <div rustbindgen hide></div>
class Foo {
    // ...
};
```
