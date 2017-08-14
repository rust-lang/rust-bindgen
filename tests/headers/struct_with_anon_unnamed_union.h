// bindgen-flags: --with-derive-hash --with-derive-partialeq
struct foo {
    union {
        unsigned int a;
        unsigned short b;
    };
};
