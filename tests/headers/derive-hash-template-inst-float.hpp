// bindgen-flags: --with-derive-hash --with-derive-partialeq --with-derive-eq
//
/// Template definition that doesn't contain float can derive hash/partialeq/eq
template <typename T>
struct foo {
    T data;
};

/// Can derive hash/partialeq/eq when instantiated with int
struct IntStr {
    foo<int> a;
};

/// Cannot derive hash/eq when instantiated with float but can derive partialeq
struct FloatStr {
    foo<float> a;
};
