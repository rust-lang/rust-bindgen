// bindgen-flags: --with-derive-hash --with-derive-partialeq --with-derive-eq
//
struct foo {
    struct {
        int a;
        int b;
    } bar;
};
