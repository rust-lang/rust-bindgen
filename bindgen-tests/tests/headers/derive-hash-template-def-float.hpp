// bindgen-flags: --with-derive-hash --with-derive-partialord --with-derive-ord --with-derive-partialeq --with-derive-eq
//
/// Template definition containing a float, which cannot derive Hash/Eq/Ord but can derive PartialEq/PartialOrd.
template <typename T>
struct foo {
    T data;
    float b;
};
