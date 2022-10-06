// bindgen-flags: --with-derive-hash --with-derive-partialeq --with-derive-eq
//
union foo {
    int a;
    struct {
        int b : 7;
        int c : 25;
    };
};
