// bindgen-flags: --rust-target 1.0 --with-derive-hash --with-derive-partialeq --with-derive-eq

union foo {
    struct {
        unsigned int a;
        unsigned int b;
    } bar;
};
