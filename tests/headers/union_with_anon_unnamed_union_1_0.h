// bindgen-flags: --rust-target 1.0 --with-derive-hash --with-derive-partialeq --with-derive-eq

union foo {
    unsigned int a;
    union {
        unsigned short b;
        unsigned char c;
    };
};
