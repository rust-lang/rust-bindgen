// bindgen-flags: --with-derive-hash --with-derive-partialeq --with-derive-eq
//
/// A struct containing an array of floats that cannot derive hash/eq but can derive partialeq.
struct foo {
    float bar[3];
};
