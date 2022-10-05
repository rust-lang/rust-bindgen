// bindgen-flags: --with-derive-hash --with-derive-partialeq --with-derive-eq
struct foo {
    union {
        unsigned int a;
        unsigned short b;
    };
};
