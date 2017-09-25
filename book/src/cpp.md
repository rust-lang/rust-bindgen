# Generating Bindings to C++

`bindgen` can handle a some C++ features, but not all of them. To set
expectations: `bindgen` will give you the type definitions and FFI declarations
you need to build an API to the C++ library, but using those types in Rust will
be nowhere near as nice as using them in C++. You will have to manually call
constructors, destructors, overloaded operators, etc yourself.

When passing in header files, the file will automatically be treated as C++ if
it ends in `.hpp`. If it doesn't, adding `-x c++` clang args can be used to
force C++ mode. You probably also want to use `-std=c++14` or similar clang args
as well.

You pretty much **must** use [whitelisting](./whitelisting.html) when working
with C++ to avoid pulling in all of the `std::*` types, many of which `bindgen`
cannot handle. Additionally, you may want to mark other types as
[opaque](./opaque.html) that `bindgen` stumbles on. It is recommended to mark
all of `std::*` opaque, and to whitelist only precisely the functions and types
you intend to use.

You should read up on the [FAQs](./faq.html) as well.

## Supported Features

* Inheritance (for the most part; there are
  [some outstanding bugs](https://github.com/rust-lang-nursery/rust-bindgen/issues/380))

* Methods

* Bindings to constructors and destructors (but they aren't implicitly or
  automatically invoked)

* Function and method overloading

* Templates *without* specialization. You should be able to access individual
  fields of the class or struct.

## Unsupported Features

When `bindgen` finds a type that is too difficult or impossible to translate
into Rust, it will automatically treat it as an opaque blob of bytes. The
philosophy is that

1. we should always get layout, size, and alignment correct, and

2. just because one type uses specialization, that shouldn't cause `bindgen` to
   give up on everything else.

Without further ado, here are C++ features that `bindgen` does not support or
cannot translate into Rust:

* Inline functions and methods: see
["Why isn't `bindgen` generating bindings to inline functions?"](./faq.html#why-isnt-bindgen-generating-bindings-to-inline-functions)

* Template functions, methods of template classes and structs. We don't know
  which monomorphizations exist, and can't create new ones because we aren't a
  C++ compiler.

* Anything related to template specialization:
  * Partial template specialization
  * Traits templates
  * Specialization Failure Is Not An Error (SFINAE)

* Cross language inheritance, for example inheriting from a Rust struct in C++.

* Automatically calling copy and/or move constructors or destructors. Supporting
  this isn't possible with Rust's move semantics.
