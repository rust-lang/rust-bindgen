// bindgen-flags: --rust-target 1.0

struct foo {
    union {
        unsigned int a;
        unsigned short b;
    } bar;
};
