// bindgen-flags: --rust-target 1.0 --with-derive-hash --with-derive-partialeq

struct foo {
    union {
        unsigned int a;
        unsigned short b;
    };
};
