# Generating Bindings to Objective-C

`bindgen` does not (yet) have full objective-c support but it can generate bindings
for a lot of the apple frameworks without too much blocklisting.

In order to generate bindings, you will need `-x objective-c` as the clang
args. If you'd like to use [block](https://crates.io/crates/block) you will need
`-fblocks` as a clang arg as well.

Depending on your setup, you may need `--generate-block` to generate the block
function aliases and `--block-extern-crate` to insert a `extern crate block` at
the beginning of the generated bindings. The same logic applies to the
`--objc-extern-crate` parameter.

The objective-c classes will be represented as a `struct Foo(id)` and a trait
`IFoo` where `Foo` is the objective-c class and `id` is an alias for `*mut
objc::runtime::Object` (the pointer to the objective-c instance). The trait
`IFoo` is needed to allow for the generated inheritance.

Functions that use or return objective-c pointers of instance `Foo` will return
`Foo`. The reason this works is because `Foo` represented as `transparent`.
This will be helpful for a lot of objective-c frameworks however there are some
cases where functions return `instancetype` which is a type alias for `id` so
an occasional `foo.0` may be required. An example of this would in the UIKit
framework should you want to add a `UILabel` to a
[UIStackView](https://developer.apple.com/documentation/uikit/uistackview/1616227-addarrangedsubview?language=objc)
you will need to convert the `UILabel` to a `UIView` via `UIView(label.0)`.

Each class (struct) has an `alloc` and a `dealloc` to match that of some of the alloc
methods found in `NSObject`.

In order to initialize a class `Foo`, you will have to do something like `let
foo = Foo(Foo::alloc().initWithStuff())`.

To blocklist an Objective-C method, you should add the bindgen generated method
path (e.g. `IFoo::method` or `IFoo::class_method`) as a blocklist item.

## Supported Features

* Inheritance matched to rust traits with prefixes of `I` which
stands for interface.
* Protocols which match to rust traits with prefixes of `P` which
stands for Protocol.
* Classes will generate `struct Foo(id)` where `Foo` is the class
name and `id` is a pointer to the objective-c Object.
* Blocks

## Useful Notes

* If you're targeting `aarch64-apple-ios`, you'll need to have the clang arg
`--target=arm64-apple-ios` as mentioned
[here](https://github.com/rust-lang/rust-bindgen/issues/1211#issuecomment-569804287).
* The generated bindings will almost certainly have some conflicts so you will
have to blocklist a few things. There are a few cases of the parameters being
poorly named in the objective-c headers. But if you're using anything with
Core Foundation, you'll find that `time.h` as has a variable called timezone that
conflicts with some of the things in `NSCalendar.h`.
* Some small subset of the function headers in the apple frameworks go against
apple's guidelines for parameter names and duplicate the names in the header
which won't compile as mentioned
[here](https://github.com/rust-lang/rust-bindgen/issues/1705).
* instancetype return methods does not return `Self` for you given class, it
returns a `mut * objc::runtime::Objc` which is aliased as `id`. This is because
objective-c's inheritance doesn't perfectly match that of rusts.
* Depending on what you're trying `bindgen` against, you may end up including
all of Core Foundation and any other frameworks. This will result in a very
long compile time.

## Not (yet) Supported

* Nullability attributes which return `Option`s.
* Probably many other things. Feel free to [open an issue](https://github.com/rust-lang/rust-bindgen/issues).

# Example crate(s)

* [uikit-sys](https://github.com/simlay/uikit-sys)
