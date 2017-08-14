// bindgen-flags: --with-derive-hash

union foo {
    struct {
        unsigned int a;
        unsigned int b;
    } bar;
};
