// bindgen-flags: --with-derive-hash
template<typename T>
struct Wrapper {
    struct Wrapped {
        T t;
    };
    using Type = Wrapped;
};
