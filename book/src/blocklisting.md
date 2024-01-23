# Blocklisting

If you need to provide your own custom translation of some type (for example,
because you need to wrap one of its fields in an `UnsafeCell`), you can
explicitly blocklist generation of its definition. Uses of the blocklisted type
will still appear in other types' definitions. (If you don't want the type to
appear in the bindings at
all, [make it opaque](./opaque.md) instead of
blocklisting it.)

Blocklisted types are pessimistically assumed not to be able to `derive` any
traits, which can transitively affect other types' ability to `derive` traits or
not.

The `blocklist-file` option also allows the blocklisting of all items from a
particular path regex, for example to block all types defined in system headers
that are transitively included.

### Library

* [`bindgen::Builder::blocklist_file`](https://docs.rs/bindgen/latest/bindgen/struct.Builder.html#method.blocklist_file)
* [`bindgen::Builder::blocklist_function`](https://docs.rs/bindgen/latest/bindgen/struct.Builder.html#method.blocklist_function)
* [`bindgen::Builder::blocklist_item`](https://docs.rs/bindgen/latest/bindgen/struct.Builder.html#method.blocklist_item)
* [`bindgen::Builder::blocklist_type`](https://docs.rs/bindgen/latest/bindgen/struct.Builder.html#method.blocklist_type)
* [`bindgen::Builder::blocklist_var`](https://docs.rs/bindgen/latest/bindgen/struct.Builder.html#method.blocklist_var)

### Command Line

* `--blocklist-file <path>`
* `--blocklist-function <function>`
* `--blocklist-item <item>`
* `--blocklist-type <type>`
* `--blocklist-var <var>`


### Annotations

```cpp
/// <div rustbindgen hide></div>
class Foo {
    // ...
};
```
