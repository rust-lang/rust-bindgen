// bindgen-flags: --with-derive-hash --with-derive-partialeq
struct foo {
    struct {
        int a;
        int b;
    } *bar;
};
