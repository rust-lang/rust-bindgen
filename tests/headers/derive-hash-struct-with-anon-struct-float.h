// bindgen-flags: --with-derive-hash --with-derive-partialord --with-derive-ord --with-derive-partialeq --with-derive-eq
//
/// A struct containing a struct containing a float that cannot derive Hash/Eq/Ord but can derive PartialEq/PartialOrd
struct foo {
    struct {
        float a;
        float b;
    } bar;
};
