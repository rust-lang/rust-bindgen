// bindgen-flags: --with-derive-hash --with-derive-partialord --with-derive-ord --with-derive-partialeq --with-derive-eq
//
/// Template definition that doesn't contain float can derive Hash/PartialOrd/Ord/PartialEq/Eq
template <typename T>
struct foo {
    T data;
};

/// Can derive Hash/PartialOrd/Ord/PartialEq/Eq when instantiated with int
struct IntStr {
    foo<int> a;
};

/// Cannot derive Hash/Eq/Ord when instantiated with float but can derive PartialEq/PartialOrd
struct FloatStr {
    foo<float> a;
};
