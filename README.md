# Servo's rust-bindgen

A binding generator for the Rust language.

This is a fork of [crabtw/rust-bindgen](https://github.com/crabtw/rust-bindgen)
designed to work on C++ code as well.

Currently this is being used for Servo's SpiderMonkey bindings, and also for
the [Stylo](https://public.etherpad-mozilla.org/p/stylo) project.

## Requirements

The current generator runs on with clang 3.8, but can also run with clang 3.9
with more features (such as detection of inlined functions).

### Installing clang 3.8

#### OSX

```
# brew install llvm38
```

#### On Debian-based Linuxes

```
# apt-get install llvm-3.8-dev libclang-3.8-dev
```

#### Arch

```
# pacman -S clang clang-tools-extra
```

### Getting clang 3.9

#### From a package manager

Clang 3.9 has ben released about a month ago, and some package managers already
provide it.

For example, for MacPorts:

```
$ port install clang-3.9
$ LIBCLANG_PATH=/opt/local/libexec/llvm-3.9/lib \
  LD_LIBRARY_PATH=/opt/local/libexec/llvm-3.9/lib \
  cargo build
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

Just run:

```
cargo build
```

This version of bindgen requires LLVM 3.9, so if you don't have it installed in
your `PATH`, you'll need to build it (see below) and do something like:

```
$ LIBCLANG_PATH=path/to/clang-3.9/build/lib \
  LD_LIBRARY_PATH=path/to/clang-3.9/build/lib \
    cargo build
```

## Running Tests

```
$ make test
```

# Command Line Usage

There are a few options documented when running `./bindgen --help`. Other
options might exist (see [the SpiderMonkey script][sm-script] and [the Stylo
scripts][stylo-scripts] to see how is it used inside the Servo organisation.

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

# Macro Usage

This mode isn't actively maintained, so no promises are made around it. Check
out the upstream documentation for info about how it *should* work.

[sm-script]: https://github.com/servo/rust-mozjs/blob/master/etc/bindings.sh
[stylo-scripts]: https://github.com/servo/servo/tree/master/ports/geckolib/gecko_bindings/tools
