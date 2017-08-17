// bindgen-flags:  --with-derive-hash --with-derive-partialeq --with-derive-eq -- -std=c++14

template <typename T, typename>
struct Foo {
    T t_member;
};

template <typename U, typename>
struct Bar {
    char member;
};

template <typename, typename V>
struct Quux {
    V v_member;
};

template <typename, typename W>
struct Lobo {
    char also_member;
};

template <typename>
using AliasWithAnonType = char;
