// bindgen-flags: --with-derive-hash --with-derive-partialeq
struct a {
    struct b* val_a;
};

struct b {
    int val_b;
};
