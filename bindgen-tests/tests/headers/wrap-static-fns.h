// bindgen-flags: --experimental --wrap-static-fns

static inline int foo() {
    return 11;
}
static int bar() {
    return 1;
}
inline int baz() {
    return 2;
}
