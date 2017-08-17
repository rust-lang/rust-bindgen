// bindgen-flags: --with-derive-hash --with-derive-partialeq --with-derive-eq
template<typename T>
struct Wrapper {
    struct Wrapped {
        T t;
    };
    using Type = Wrapped;
};
