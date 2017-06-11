// bindgen-flags: --constified-enum-module foo

enum foo {
  THIS,
  SHOULD_BE,
  A_CONSTANT,
};

struct bar {
  enum foo this_should_work;
};
