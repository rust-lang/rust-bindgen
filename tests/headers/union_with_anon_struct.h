// bindgen-flags: --with-derive-hash --with-derive-partialeq
//
union foo {
    struct {
        unsigned int a;
        unsigned int b;
    } bar;
};
