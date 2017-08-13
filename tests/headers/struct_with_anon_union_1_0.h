// bindgen-flags: --rust-target 1.0 --with-derive-hash

struct foo {
    union {
        unsigned int a;
        unsigned short b;
    } bar;
};
