# Generating Bindings to C++

`bindgen` can handle some C++ features, but not all of them. To set
expectations: `bindgen` will give you the type definitions and FFI declarations
you need to build an API to the C++ library, but using those types in Rust will
be nowhere near as nice as using them in C++. You will have to manually call
constructors, destructors, overloaded operators, etc yourself.

When passing in header files, the file will automatically be treated as C++ if
it ends in `.hpp`. If it doesn't, adding `-x c++` clang args can be used to
force C++ mode. You probably also want to use `-std=c++14` or similar clang args
as well.

You pretty much **must** use [allowlisting](./allowlisting.md) when working
with C++ to avoid pulling in all of the `std::.*` types, many of which `bindgen`
cannot handle. Additionally, you may want to mark other types as
[opaque](./opaque.md) that `bindgen` stumbles on. It is recommended to mark
all of `std::.*` opaque, and to allowlist only precisely the functions and types
you intend to use.

You should read up on the [FAQs](./faq.md) as well.

## Supported Features

* Inheritance (for the most part; there are
  [some outstanding bugs](https://github.com/rust-lang/rust-bindgen/issues/380))

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
["Why isn't `bindgen` generating bindings to inline functions?"](./faq.md#why-isnt-bindgen-generating-bindings-to-inline-functions)

* Template functions, methods of template classes and structs. We don't know
  which monomorphizations exist, and can't create new ones because we aren't a
  C++ compiler.

* Anything related to template specialization:
  * Partial template specialization
  * Traits templates
  * Substitution Failure Is Not An Error (SFINAE)

* Cross language inheritance, for example inheriting from a Rust struct in C++.

* Automatically calling copy and/or move constructors or destructors. Supporting
  this isn't possible with Rust's move semantics.

* Exceptions: if a function called through a `bindgen`-generated interface
  raises an exception that is not caught by the function itself, this will
  generate undefined behaviour. See
  [the tracking issue for exceptions](https://github.com/rust-lang/rust-bindgen/issues/1208)
  for more details.
  
* Many C++ specific aspects of calling conventions. For example in the Itanium abi types that are 
  "[non trivial for the purposes of calls](https://itanium-cxx-abi.github.io/cxx-abi/abi.html#non-trivial)" 
  should be passed by pointer, even if they are otherwise eligible to be passed in a register.
  Similarly in both the Itanium and MSVC ABIs such types are returned by "hidden parameter", much like
  large structs in C that would not fit into a register. This also applies to types with any base classes
  in the MSVC ABI (see [x64 calling convention](https://learn.microsoft.com/en-us/cpp/build/x64-calling-convention?view=msvc-170#return-values)).
  Because bindgen does not know about these rules generated interfaces using such types are currently invalid.

## Constructor semantics

`bindgen` will generate a wrapper for any class constructor declared in the
input headers. For example, this headers file

```c++
class MyClass {
    public:
	MyClass();
        void method();
};
```

Will produce the following code:
```rust,ignore
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct MyClass {
    pub _address: u8,
}
extern "C" {
    #[link_name = "\u{1}_ZN7MyClass6methodEv"]
    pub fn MyClass_method(this: *mut MyClass);
}
extern "C" {
    #[link_name = "\u{1}_ZN7MyClassC1Ev"]
    pub fn MyClass_MyClass(this: *mut MyClass);
}
impl MyClass {
    #[inline]
    pub unsafe fn method(&mut self) {
        MyClass_method(self)
    }
    #[inline]
    pub unsafe fn new() -> Self {
        let mut __bindgen_tmp = ::std::mem::MaybeUninit::uninit();
        MyClass_MyClass(__bindgen_tmp.as_mut_ptr());
        __bindgen_tmp.assume_init()
    }
}
```
This `MyClass::new` Rust method can be used as a substitute for the `MyClass`
C++ constructor. However, the address of the value from inside the method will
be different than from the outside. This is because the `__bindgen_tmp` value
is moved when the `MyClass::new` method returns.

In contrast, the C++ constructor will not move the value, meaning that the
address of the value will be the same inside and outside the constructor.
If the original C++ relies on this semantic difference somehow, you should use the
`MyClass_MyClass` binding directly instead of the `MyClass::new` method.

In other words, the Rust equivalent for the following C++ code

```c++
MyClass instance = MyClass();
instance.method();
```

is not this

```rust,ignore
let instance = MyClass::new();
instance.method();
```

but this

```rust,ignore
let instance = std::mem::MaybeUninit::<MyClass>::uninit();
MyClass_MyClass(instance.as_mut_ptr());
instance.assume_init_mut().method();
```

You can easily verify this fact if you provide a implementation for `MyClass`
and `method` that prints the the `this` pointer address. However, you can
ignore this fact if you know that the original C++ code does not rely on the
instance address in its internal logic.
