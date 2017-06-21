// bindgen-flags: --constified-enum-module .*

typedef enum foo {
  THIS,
  SHOULD_BE,
  A_CONSTANT,
  ALSO_THIS = 42,
  AND_ALSO_THIS = 42,
} foo;


typedef enum {
  Variant1, Variant2, Variant3,
} anon_enum;


namespace ns1 {
  typedef enum {
    THIS,
    SHOULD_BE,
    A_CONSTANT,
    ALSO_THIS = 42,
  } foo;
}

namespace ns2 {
  enum class Foo {
    Variant1,
    Variant2,
  };
}

typedef foo foo_alias1;
typedef foo_alias1 foo_alias2;
typedef foo_alias2 foo_alias3;

typedef anon_enum anon_enum_alias1;
typedef anon_enum_alias1 anon_enum_alias2;
typedef anon_enum_alias2 anon_enum_alias3;

typedef struct bar {
  foo member1;
  foo_alias1 member2;
  foo_alias2 member3;
  foo_alias3 member4;
  ns1::foo member5;
  ns2::Foo *member6;
  anon_enum member7;
  anon_enum_alias1 member8;
  anon_enum_alias2 member9;
  anon_enum_alias3 member10;
} bar;

class Baz {
  ns2::Foo member1;
};

namespace one {
  enum class Foo {
    Variant1, Variant2,
  };
}

class Bar {
  one::Foo* baz;
};

foo *func1(foo arg1, foo *arg2, foo **arg3);
foo_alias1 *func2(foo_alias1 arg1, foo_alias1 *arg2, foo_alias1 **arg3);

template <class T>
class Thing {
  T thing;
  T& get_thing();
};

foo func3(Thing<foo> arg1);
foo func4(Thing< Thing<foo> > arg1);