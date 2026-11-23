// bindgen-flags: --allowlist-type StructWithBitfieldAndMethod -- -std=c++20

struct Base {
    template<typename T> void t_func(T t);
};

struct StructWithBitfieldAndMethod : Base {
    int field : 1;
    template<typename T> void t_method(T t);
    void regular_method();
};
