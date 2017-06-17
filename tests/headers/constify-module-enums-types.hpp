// bindgen-flags: --constified-enum-module foo

typedef enum foo {
  THIS,
  SHOULD_BE,
  A_CONSTANT,
  ALSO_THIS = 42,
  AND_ALSO_THIS = 42,
} foo;

namespace ns1 {
  typedef enum foo2 {
    THIS,
    SHOULD_BE,
    A_CONSTANT,
    ALSO_THIS = 42,
  } foo;
}

typedef foo foo_alias1;
typedef foo_alias1 foo_alias2;

typedef struct bar {
  foo member1;
  foo_alias1 member2;
  foo_alias2 member3;
  ns1::foo member4;
} bar;

foo *func1(foo arg1, foo *arg2, foo **arg3);
foo_alias1 *func2(foo_alias1 arg1, foo_alias1 *arg2, foo_alias1 **arg3);

template <class T>
class Thing {
  T thing;
  T& get_thing();
};

foo func3(Thing<foo> arg1);