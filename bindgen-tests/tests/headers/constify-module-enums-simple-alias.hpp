// bindgen-flags: --constified-enum-module Foo

enum class Foo {
  Variant1, Variant2, Variant3,
};

typedef Foo Foo_alias1;
typedef Foo_alias1 Foo_alias2;
typedef Foo_alias2 Foo_alias3;

class Bar {
  Foo baz1;
  Foo_alias1 baz2;
  Foo_alias2 baz3;
  Foo_alias3 baz4;

  Foo *baz_ptr1;
  Foo_alias1 *baz_ptr2;
  Foo_alias2 *baz_ptr3;
  Foo_alias3 *baz_ptr4;
};
