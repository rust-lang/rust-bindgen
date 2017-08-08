// bindgen-flags: --with-derive-hash
//
/// Template definition containing a float, which cannot derive hash.
template<typename T>
struct foo {
    T data;
    float b;
};
