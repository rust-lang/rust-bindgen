// bindgen-flags: --with-derive-hash --with-derive-partialeq --with-derive-eq
//
struct foo {
    struct {
        unsigned int x;
        unsigned int y;
    } bar;
};
