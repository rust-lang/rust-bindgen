// bindgen-flags: --with-derive-hash
//
struct foo {
    union {
        unsigned int a;
        unsigned short b;
    } bar;
};
