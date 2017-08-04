// bindgen-flags: --rust-target 1.0

union foo {
    union {
        unsigned int a;
        unsigned short b;
    } bar;
};
