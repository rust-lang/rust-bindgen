// bindgen-flags: -- -std=c++14

// Filtering out the non-argument children of a function declaration must not
// change how ordinary template instantiations, or instantiations with const
// value arguments, are handled when the function has parameters.

class Foo;

template <typename T>
struct Box {
  T* ptr;
};

template <typename T, int N>
struct Array {
  T items[N];
};

template <typename T>
using Quux = T*;

// A class template instantiation rather than an alias.
Box<Foo> box_result(int x);

// Nested instantiations of both kinds.
Box<Quux<Foo>> nested_result(int x, int y);
Quux<Box<Foo>> alias_of_box_result(int x);

// Const value arguments are still unsupported and must keep being skipped
// rather than being emitted with the wrong arguments.
Array<Foo, 3> const_arg_result(int x);

struct WithInlineBody {
  Box<Foo> in_class_body(int x) {
    return Box<Foo>();
  }
  Quux<Foo> declared_only(int x);
};
