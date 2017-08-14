// bindgen-flags: --with-derive-hash

union foo {
    union {
        unsigned int a;
        unsigned short b;
    } bar;
};
