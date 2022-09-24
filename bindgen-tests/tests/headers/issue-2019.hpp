// bindgen-flags: --disable-nested-struct-naming

struct A {
    static A make();
    int a;
};
struct B {
    static B make();
    int b;
};
