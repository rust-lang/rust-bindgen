// bindgen-flags: --with-derive-hash
//
/// A struct containing an array of floats that cannot derive hash.
struct foo {
    float bar[3];
};
