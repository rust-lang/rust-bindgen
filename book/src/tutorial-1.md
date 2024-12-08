# Add `bindgen` as a Build Dependency

First we need to declare a build-time dependency on `bindgen` by adding it to
the `[build-dependencies]` section of our crate's `Cargo.toml` file.

Please always use the latest version of `bindgen`, as it has the most fixes and
best compatibility.
You can always check the latest version at
[the bindgen page in crates.io](https://crates.io/crates/bindgen).

```toml
[build-dependencies]
bindgen = "0.71.0"
```

> ⚠️ **Warning**
>
> `bindgen` needs to be added to the `[build-dependencies]` section, not the normal
> `[dependencies]` section. If you add it as a regular dependency, you will get
> errors like the following: `` error[E0463]: can't find crate for `bindgen` ``
