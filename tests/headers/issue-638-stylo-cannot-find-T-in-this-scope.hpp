// bindgen-flags: -- -std=c++14

template <class T>
class RefPtr {
public:
    T use_of_t;
};

template <typename U>
class UsesRefPtrWithAliasedTypeParam {
public:
    using V = U;
    RefPtr<V> member;
};
