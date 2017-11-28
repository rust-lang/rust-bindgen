// bindgen-flags: --constified-enum-module one::Foo

namespace one {
  enum class Foo {
    Variant1, Variant2,
  };
}

class Bar {
  one::Foo baz1;
  one::Foo* baz2;
};
