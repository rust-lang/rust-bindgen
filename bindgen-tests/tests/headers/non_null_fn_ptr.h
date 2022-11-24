// bindgen-flags: --non-null-fn-ptr=".*foo::.*" --no-default=".*foo"

typedef struct foo {
    int (*bar)();
} foo;

foo new_foo(int (*arg)());

typedef struct baz {
    int (*foo)();
} baz;

baz new_baz(int (*foo)());

typedef union union_foo {
    int (*fst)();
    float (*snd)();
} union_foo;

union_foo new_union_foo(int (*arg)());

typedef union union_baz {
    int (*foo)();
    float (*bar)();
} union_baz;

union_baz new_union_baz(int (*foo)());
