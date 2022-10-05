// bindgen-flags: --with-derive-hash --with-derive-partialeq --with-derive-eq
//
union foo {
    struct {
        unsigned int a;
        unsigned int b;
    } bar;
};
