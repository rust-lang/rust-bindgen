// bindgen-flags: --with-derive-hash --with-derive-partialord --with-derive-partialeq --with-derive-eq
//
/// A struct containing a struct containing a float that cannot derive hash/eq but can derive partialeq and partialord
struct foo {
    struct {
        float a;
        float b;
    } bar;
};
