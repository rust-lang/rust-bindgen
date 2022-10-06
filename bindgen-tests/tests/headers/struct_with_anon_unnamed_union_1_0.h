// bindgen-flags: --rust-target 1.0 --with-derive-hash --with-derive-partialeq --with-derive-eq

struct foo {
    union {
        unsigned int a;
        unsigned short b;
    };
};
