// bindgen-flags: --experimental --generate-fn-macros

// The `f1` macro call in `f2` should be expanded.
#define f1(x) x * 2
#define f2(y) y * f1(y)
