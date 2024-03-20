# Using C structures with Flexible Array Members

Since time immemorial, C programmers have been using what was called "the struct
hack". This is a technique for packing a fixed-size structure and a
variable-sized tail within the same memory allocation. Typically this looks
like:

```c
struct MyRecord {
    time_t timestamp;
    unsigned seq;
    size_t len;
    char payload[0];
};
```

Because this is so useful, it was standardized in C99 as "flexible array
members", using almost identical syntax:
```c
struct MyRecord {
    time_t timestamp;
    unsigned seq;
    size_t len;
    char payload[]; // NOTE: empty []
};
```

Bindgen supports these structures in two different ways.

## `__IncompleteArrayField`

By default, bindgen will generate the corresponding Rust structure:
```rust,ignore
#[repr(C)]
struct MyRecord {
    pub timestamp: time_t,
    pub seq: ::std::os::raw::c_uint,
    pub len: usize,
    pub payload: __IncompleteArrayField<::std::os::raw::c_char>,
}
```

The `__IncompleteArrayField` type is zero-sized, so this structure represents
the prefix without any trailing data. In order to access that data, it provides
the `as_slice` unsafe method:
```rust,ignore
    // SAFETY: there's at least `len` bytes allocated and initialized after `myrecord`
    let payload = unsafe { myrecord.payload.as_slice(myrecord.len) };
```
There's also `as_mut_slice` which does the obvious.

These are `unsafe` simply because it's up to you to provide the right length (in
elements of whatever type `payload` is) as there's no way for Rust or Bindgen to
know. In this example, the length is a very straightforward `len` field in the
structure, but it could be encoded in any number of ways within the structure,
or come from somewhere else entirely.

One big caveat with this technique is that `std::mem::size_of` (or
`size_of_val`) will *only* include the size of the prefix structure. if you're
working out how much storage the whole structure is using, you'll need to add
the suffix yourself.

## Using Dynamically Sized Types

If you invoke bindgen with the `--flexarray-dst` option, it will generate
something not quite like this:

```rust,ignore
#[repr(C)]
struct MyRecord {
    pub timestamp: time_t,
    pub seq: ::std::os::raw::c_uint,
    pub len: usize,
    pub payload: [::std::os::raw::c_char],
}
```
Rust has a set of types which are almost exact analogs for these Flexible Array
Member types: the Dynamically Sized Type ("DST").

This looks almost identical to a normal Rust structure, except that you'll note
the type of the `payload` field is a raw slice `[...]` rather than the usual
reference to slice `&[...]`.

That `payload: [c_char]` is telling Rust that it can't directly know the total
size of this structure - the `payload` field takes an amount of space that's
determined at runtime. This means you can't directly use values of this type,
only references: `&MyRecord`.

In practice, this is very awkward. So instead, bindgen generates:
```rust,ignore
#[repr(C)]
struct MyRecord<FAM: ?Sized = [::std::os::raw::c_char; 0]> {
    pub timestamp: time_t,
    pub seq: ::std::os::raw::c_uint,
    pub len: usize,
    pub payload: FAM,
}
```

That is:
1. a type parameter `FAM` which represents the type of the `payload` field,
2. it's `?Sized` meaning it can be unsized (ie, a DST)
3. it has the default type of `[c_char; 0]` - that is a zero-sized array of characters

This means that referencing plain `MyRecord` will be exactly like `MyRecord`
with `__IncompleteArrayField`: it is a fixed-sized structure which you can
manipulate like a normal Rust value.

But how do you get to the DST part?

Bindgen will also implement a set of helper methods for this:

```rust,ignore
// Static sized variant
impl MyRecord<[::std::os::raw::c_char; 0]> {
    pub unsafe fn flex_ref(&self, len: usize) -> &MyRecord<[::std::os::raw::c_char]> { ... }
    pub unsafe fn flex_mut_ref(&mut self, len: usize) -> &mut MyRecord<[::std::os::raw::c_char]> { ... }
    // And some raw pointer variants
}
```
These will take a sized `MyRecord<[c_char; 0]>` and a length in elements, and
return a reference to a DST `MyRecord<[c_char]>` where the `payload` field is a
fully usable slice of `len` characters.

The magic here is that the reference is a fat pointer, which not only encodes
the address, but also the dynamic size of the final field, just like a reference
to a slice is. This means that you get full bounds checked access to the
`payload` field like any other Rust slice.

It also means that doing `mem::size_of_val(myrecord)` will return the *complete*
size of this structure, including the suffix.

You can go the other way:
```rust,ignore
// Dynamic sized variant
impl MyRecord<[::std::os::raw::c_char]> {
    pub fn fixed(&self) -> (&MyRecord<[::std::os::raw::c_char; 0]>, usize) { ... }
    pub fn fixed_mut(&mut self) -> (&mut MyRecord<[::std::os::raw::c_char; 0]>, usize) { ... }
    pub fn layout(len: usize) -> std::alloc::Layout { ... }
}
```
which takes the DST variant of the structure and returns the sized variant,
along with the number of elements are after it. These are all completely safe
because all the information needed is part of the fat `&self` reference.

The `layout` function takes a length and returns the `Layout` - that is, size
and alignment, so that you can allocate memory for the structure (for example,
using `malloc` so you can pass it to a C function).

Unfortunately the language features needed to support these methods are still unstable:
- [ptr_metadata](https://doc.rust-lang.org/beta/unstable-book/library-features/ptr-metadata.html),
  which enables all the fixed<->DST conversions, and
- [layout_for_ptr](https://doc.rust-lang.org/beta/unstable-book/library-features/layout-for-ptr.html),
  which allows he `layout` method

As a result, if you don't specify `--rust-target nightly` you'll just get the
bare type definitions, but no real way to use them. It's often convenient to add
the
```bash
--raw-line '#![feature(ptr_metadata,layout_for_ptr)]'
```
option if you're generating Rust as a stand-alone crate. Otherwise you'll need
to add the feature line to your containing crate.
