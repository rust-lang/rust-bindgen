// bindgen-flags: --with-derive-hash --with-derive-partialord --with-derive-partialeq --with-derive-eq
//
/// Template definition containing a float, which cannot derive hash/eq but can derive partialeq and partialord.
template <typename T>
struct foo {
    T data;
    float b;
};
