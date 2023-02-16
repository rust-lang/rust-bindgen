// bindgen-flags: --experimental --wrap-static-fns

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
