// bindgen-flags: --with-derive-hash --with-derive-partialeq
//
union foo {
    unsigned int a;
    struct {
        union {
            unsigned short b1;
            unsigned short b2;
        };

        union {
            unsigned short c1;
            unsigned short c2;
        };
    };
};
