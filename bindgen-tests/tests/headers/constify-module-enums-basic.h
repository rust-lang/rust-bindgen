// bindgen-flags: --constified-enum-module foo

enum foo {
  THIS,
  SHOULD_BE,
  A_CONSTANT,
};

typedef enum foo foo_alias1;
typedef foo_alias1 foo_alias2;

struct bar {
  enum foo this_should_work;
};

enum foo *func1(enum foo arg1, enum foo *arg2, enum foo **arg3);
foo_alias1 *func2(foo_alias1 arg1, foo_alias1 *arg2, foo_alias1 **arg3);