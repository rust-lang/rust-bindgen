// bindgen-flags: --translate-function-macros

#define A(x) B(x)
#define B(x) ((x) + 1)
