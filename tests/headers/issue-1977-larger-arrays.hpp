// bindgen-flags: --with-derive-hash --with-derive-partialeq --with-derive-eq
//
struct S {
    char large_array[33];
};

template<typename T> struct ST {
    T large_array[33];
};
