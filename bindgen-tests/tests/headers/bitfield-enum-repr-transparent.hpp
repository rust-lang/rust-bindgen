// bindgen-flags: --bitfield-enum "Foo"

enum Foo {
  Bar = 1 << 1,
  Baz = 1 << 2,
  Duplicated = 1 << 2,
  Negative = -3,
};
