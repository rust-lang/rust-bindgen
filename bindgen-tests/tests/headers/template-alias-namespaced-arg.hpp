// bindgen-flags: -- -std=c++14

// Namespace qualifiers in front of a template argument show up as
// `NamespaceRef` cursors next to the `TypeRef`. They are not template
// arguments themselves and must not make the whole instantiation get dropped.

namespace baz {
class Foo;
namespace inner {
class Bar;
}
}

template <typename T>
using Quux = T*;

template <typename T, typename U>
struct Pair {
  T* first;
  U* second;
};

Quux<baz::Foo> global;
Quux<baz::inner::Bar> nested_global;

Quux<baz::Foo> returns_alias(int x);
void takes_alias(Quux<baz::Foo> a, int x);

Pair<baz::Foo, baz::inner::Bar> pair_global;
Pair<baz::Foo, baz::inner::Bar> returns_pair(int x);
