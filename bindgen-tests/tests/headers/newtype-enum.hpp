// bindgen-flags: --newtype-enum "Foo" --rust-target 1.28  -- -std=c++11

enum Foo {
  Bar = 1 << 1,
  Baz = 1 << 2,
  Duplicated = 1 << 2,
  Negative = -3,
};
