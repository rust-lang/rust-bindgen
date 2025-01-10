It is also possible to generate bindings for loading a shared library via the `libloading` crate.
To generate libloading bindings for a library `shared_lib` we can either use the function `dynamic_library_name` function in `build.rs` or the CLI argument `--dynamic-loading` when using the bindgen CLI.

Here is an example using the bindgen CLI:
```
bindgen wrapper.h --dynamic-loading MySharedLib --output bindings.rs
```
Bindgen will generate a `libloading` struct matching the name of the `--dynamic-loading` argument. So in the case of the example above the generated struct will have the name `MySharedLib`.
