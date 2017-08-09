// bindgen-flags: --with-derive-hash
//
/// A struct containing a struct containing a float that cannot derive hash.
struct foo {
    struct {
        float a;
        float b;
    } bar;
};
