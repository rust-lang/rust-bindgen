// bindgen-flags: --rust-target 1.0 --with-derive-hash

union foo {
    struct {
        unsigned int a;
        unsigned int b;
    } bar;
};
