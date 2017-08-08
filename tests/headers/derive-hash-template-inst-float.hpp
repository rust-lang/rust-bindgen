// bindgen-flags: --with-derive-hash
//
/// Template definition that doesn't contain float can derive hash
template<typename T>
struct foo {
    T data;
};

/// Can derive hash when instantiated with int
struct IntStr {
    foo<int> a;
};

/// Cannot derive hash when instantiated with float
struct FloatStr {
    foo<float> a;
};
