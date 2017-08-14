// bindgen-flags: --with-derive-hash --with-derive-partialeq
template<typename T>
struct Wrapper {
    struct Wrapped {
        T t;
    };
    using Type = Wrapped;
};
