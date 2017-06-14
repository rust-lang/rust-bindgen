// bindgen-flags: --constified-enum-module foo

typedef enum foo {
  THIS,
  SHOULD_BE,
  A_CONSTANT,
  ALSO_THIS = 42,
  AND_ALSO_THIS = 42,
} foo;

typedef foo foo_alias1;
typedef foo_alias1 foo_alias2;

typedef struct bar {
  foo member1;
  foo_alias1 member2;
  foo_alias2 member3;
} bar;
