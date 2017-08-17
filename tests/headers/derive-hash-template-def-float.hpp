// bindgen-flags: --with-derive-hash --with-derive-partialeq --with-derive-eq
//
/// Template definition containing a float, which cannot derive hash/eq but can derive partialeq.
template <typename T>
struct foo {
    T data;
    float b;
};
