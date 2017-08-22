// bindgen-flags: --with-derive-hash --with-derive-partialeq --with-derive-eq
//
/// A struct containing a struct containing a float that cannot derive hash/eq but can derive partial eq.
struct foo {
    struct {
        float a;
        float b;
    } bar;
};
