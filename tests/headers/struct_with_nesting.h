// bindgen-flags: --with-derive-hash --with-derive-partialeq --with-derive-eq
//
struct foo {
    unsigned int a;
    union {
        unsigned int b;
        struct {
            unsigned short c1;
            unsigned short c2;
        };

        struct {
            unsigned char d1;
            unsigned char d2;
            unsigned char d3;
            unsigned char d4;
        };
    };
};
