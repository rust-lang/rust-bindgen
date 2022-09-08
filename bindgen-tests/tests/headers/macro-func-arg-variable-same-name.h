// bindgen-flags: --experimental --generate-fn-macros

// The `x` argument of the `f3` function-like macro
// and the `x` in the nested `y` macro should be treated
// as different variables.
#define y x
#define f3(x) y + y
#define x 2
