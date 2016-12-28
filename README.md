# Servo's rust-bindgen

A binding generator for the Rust language.

This is a fork of [crabtw/rust-bindgen](https://github.com/crabtw/rust-bindgen)
designed to work on C++ code as well.

Currently this is being used for Servo's SpiderMonkey bindings, and also for
the [Stylo](https://public.etherpad-mozilla.org/p/stylo) project.

## Requirements

It is recommended to use clang 3.9 with the current generator. It can run with
clang 3.8 with some features disabled.

### Installing Clang 3.9

#### Windows

Download and install the official pre-built binary from
[LLVM download page](http://releases.llvm.org/download.html).

#### OSX

If you use Homebrew:
```
$ brew install llvm
```

If you use MacPorts:
```
$ port install clang-3.9
```

#### Debian-based Linuxes

```
# apt-get install llvm-3.9-dev libclang-3.9-dev
```

Ubuntu 16.10 provides the necessary packages directly. If you are using older
version of Ubuntu or other Debian-based distros, you may need to add the LLVM
repos to get version 3.9. See http://apt.llvm.org/.

#### Arch

```
# pacman -S clang
```

#### From source

If your package manager doesn't yet offer Clang 3.9, you'll need to build from
source. For that, follow the instructions
[here](http://clang.llvm.org/get_started.html).

Those instructions list optional steps. For bindgen:

* Checkout and build clang
* Checkout and build the extra-clang-tools
* Checkout and build the compiler-rt
* You do not need to checkout or build libcxx

## Building

```
$ cd bindgen
$ cargo build
```

If you installed multiple versions of llvm, it may not be able to locate the
latest version of libclang. In that case, you may want to either uninstall
other versions of llvm, or specify the path of the desired libclang explicitly:
```
$ export LIBCLANG_PATH=path/to/clang-3.9/lib
```

On Linux and macOS, you may also need to add a path to `libclang.so` (usually
the same path as above) to library search path. This can be done as below:
```
$ export LD_LIBRARY_PATH=path/to/clang-3.9/lib # for Linux
$ export DYLD_LIBRARY_PATH=path/to/clang-3.9/lib # for macOS
```

# Library usage with `build.rs`

See [the Stylo build script][stylo-script] to see how it is used inside the
Servo organisation.

[stylo-script]: https://github.com/servo/servo/blob/master/components/style/build_gecko.rs

In `Cargo.toml`:

```toml
[package]
# ...
build = "build.rs"

[build-dependencies]
libbindgen = "0.1"
```

In `build.rs`:

```rust
extern crate libbindgen;

use std::env;
use std::path::Path;

fn main() {
  let out_dir = env::var("OUT_DIR").unwrap();
  let _ = libbindgen::builder()
    .header("example.h")
    .use_core()
    .generate().unwrap()
    .write_to_file(Path::new(&out_dir).join("example.rs"));
}
```

In `src/main.rs`:

```rust
include!(concat!(env!("OUT_DIR"), "/example.rs"));
```

# Command Line Usage

There are a few options documented when running `./bindgen --help`. Other
options might exist (see [the SpiderMonkey script][sm-script] to see how it
is used inside the Servo organisation.

[sm-script]: https://github.com/servo/rust-mozjs/blob/master/etc/bindings.sh

## C++ Usage

This fork of rust-bindgen can handle a number of C++ features.

When passing in header files, the file will automatically be treated as C++ if
it ends in ``.hpp``. If it doesn't, ``-x c++`` can be used to force C++ mode.

## Annotations

The translation of classes, structs, enums, and typedefs can be adjusted using
annotations. Annotations are specifically formatted html tags inside doxygen
style comments.

### `opaque`

The `opaque` annotation instructs bindgen to ignore all fields defined in
a struct/class.

```cpp
/// <div rustbindgen opaque></div>
```

### `hide`

The `hide` annotation instructs bindgen to ignore the struct/class/field/enum
completely.

```
/// <div rustbindgen hide></div>
```

### `replaces`

The `replaces` annotation can be used to use a type as a replacement for other
(presumably more complex) type. This is used in Stylo to generate bindings for
structures that for multiple reasons are too complex for bindgen to understand.

For example, in a C++ header:

```cpp
/**
 * <div rustbindgen replaces="nsTArray"></div>
 */
template<typename T>
class nsTArray_Simple {
  T* mBuffer;
public:
  // The existence of a destructor here prevents bindgen from deriving the Clone
  // trait via a simple memory copy.
  ~nsTArray_Simple() {};
};
```

That way, after code generation, the bindings for the `nsTArray` type are
the ones that would be generated for `nsTArray_Simple`.

### `nocopy`

The `nocopy` annotation is used to prevent bindgen to autoderive the `Copy`
and `Clone` traits for a type.
