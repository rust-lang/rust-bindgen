// bindgen-flags: --with-derive-hash --with-derive-partialord --with-derive-ord --with-derive-partialeq --with-derive-eq
//
/// A struct containing an array of floats that cannot derive Hash/Eq/Ord but can derive PartialEq/PartialOrd
struct foo {
    float bar[3];
};
