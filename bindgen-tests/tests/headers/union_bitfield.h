// bindgen-flags: --with-derive-hash --with-derive-partialeq --with-derive-eq --impl-partialeq

union U4 {
    unsigned derp : 1;
};

union B {
    unsigned foo : 31;
    unsigned char bar : 1;
};
