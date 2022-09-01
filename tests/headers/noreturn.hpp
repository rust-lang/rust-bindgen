// bindgen-flags: --enable-function-attribute-detection
_Noreturn void f(void);
// TODO (pvdrz): figure out how to handle this case.
__attribute__((noreturn)) void g(void);
[[noreturn]] void h(void);
