// bindgen-flags: --constified-enum-module foo

enum foo {
  Type,
  Type_,
  Type1,
  Type__,
};

struct bar {
  enum foo member;
};
