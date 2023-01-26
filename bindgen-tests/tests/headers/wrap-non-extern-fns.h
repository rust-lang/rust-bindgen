// bindgen-flags: --experimental --wrap-non-extern-fns

static inline int foo() {
    return 11;
}
static int bar() {
    return 1;
}
inline int baz() {
    return 2;
}
