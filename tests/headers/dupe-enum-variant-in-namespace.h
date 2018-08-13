// bindgen-flags: --rustified-enum ".*" --enable-cxx-namespaces -- -x c++ -std=c++11

namespace foo {
  enum class Bar : unsigned {
    Foo = 0,
    Foo1 = 0,
    Foo2,
    Foo3 = Foo2,
  };
}
