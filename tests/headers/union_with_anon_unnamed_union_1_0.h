// bindgen-flags: --rust-target 1.0

union foo {
    unsigned int a;
    union {
        unsigned short b;
        unsigned char c;
    };
};
