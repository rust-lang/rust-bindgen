// bindgen-flags: -- -std=c++14

// An alias template instantiation used as a return type must keep its template
// arguments no matter what else is in the declaration: the function's
// parameters and body are siblings of the type references in the clang AST and
// must not be mistaken for template arguments.

class Foo;
class Bar;

template <typename T>
using Quux = T*;

// Only the return type is an alias template instantiation.
Quux<Foo> no_params();
Quux<Foo> one_param(int x);
Quux<Foo> two_params(int x, int y);

// The parameters are instantiations of the same alias.
Quux<Foo> alias_params(Quux<Bar> a, Quux<Foo> b);

// A parameter with a default value has children of its own.
Quux<Foo> default_param(int x = 1);

// The declaration has a body.
inline Quux<Foo> with_body(int x) {
  return 0;
}

// The return type is written after the parameters.
auto trailing_return(int x) -> Quux<Foo>;

struct Holder {
  Quux<Foo> method(int x);
  Quux<Bar> const_method(int x, int y) const;
  static Quux<Foo> static_method(int x);
};
