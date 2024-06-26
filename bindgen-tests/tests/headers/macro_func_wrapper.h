// bindgen-flags: --experimental --generate-fn-macros

void func(int, int);

#define Y 7

// This macro is invalid, since it tries to call the `func` macro with two arguments, not the `func` function.
#define wrapper_func(x) func(x, Y)

#define func(x) func(x, Y)
