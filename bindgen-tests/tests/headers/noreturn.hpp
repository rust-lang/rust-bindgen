// bindgen-flags: --enable-function-attribute-detection -- -std=c++11
_Noreturn void f(void);
__attribute__((noreturn)) void g(void);
[[noreturn]] void h(void);
