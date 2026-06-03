// bindgen-flags: --translate-function-macros

int foo(int);
#define CALL_FOO(x) (foo)(x)
