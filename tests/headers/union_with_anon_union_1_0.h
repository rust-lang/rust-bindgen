// bindgen-flags: --rust-target 1.0 --with-derive-hash

union foo {
    union {
        unsigned int a;
        unsigned short b;
    } bar;
};
