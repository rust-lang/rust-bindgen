// bindgen-flags: --experimental --wrap-static-fns
// bindgen-parse-callbacks: wrap-as-variadic-fn

// to avoid polluting the expectation tests we put the stdarg.h behind a conditional
// variable only used in bindgen-integration
#ifdef USE_VA_HEADER
#include <stdarg.h>
#endif

static inline int foo() {
    return 11;
}
static int bar();
static int bar() {
    return 1;
}
inline int baz() {
    return 2;
}

static inline int takes_ptr(int* arg) {
    return *arg + 1;
}

static inline int takes_fn_ptr(int (*f)(int)) {
    return f(1);
}

static inline int takes_fn(int (f)(int)) {
    return f(2);
}

typedef int (func)(int);

static inline int takes_alias(func f) {
    return f(3);
}

static inline int takes_qualified(const int *const *arg) {
    return **arg;
}

enum foo {
    BAR = 0x0,
};

static inline enum foo takes_enum(const enum foo f) {
    return f;
}

static inline void nevermore() {
    while (1) { }
}

static inline int takes_fn_with_no_args(int(f)(void)) {
    return f();
}

static inline int variadic(int x, ...) {
    return x;
}

static inline void no_extra_argument(__builtin_va_list va) {}

static inline int many_va_list(int i, __builtin_va_list va1, __builtin_va_list va2) {
    return i;
}

#ifndef USE_VA_HEADER
static inline int wrap_as_variadic_fn1(int i, __builtin_va_list va) {
    return i;
}

static inline void wrap_as_variadic_fn2(int i, __builtin_va_list va) {}
#else
static inline int wrap_as_variadic_fn1(int i, va_list va) {
    int res = 0;

    for (int j = 0; j < i; j++)
        res += (int) va_arg(va, int);

    return res;
}

static inline void wrap_as_variadic_fn2(int i, va_list va) {}
#endif
