// bindgen-flags: --rust-target 1.0 --with-derive-hash --with-derive-partialeq --with-derive-eq --impl-partialeq

union U4 {
    unsigned derp : 1;
};

union B {
    unsigned foo : 31;
    unsigned char bar : 1;
};

union HasBigBitfield {
    __int128 x : 128;
};
