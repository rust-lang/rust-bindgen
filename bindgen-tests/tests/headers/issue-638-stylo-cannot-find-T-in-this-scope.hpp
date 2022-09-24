// bindgen-flags: -- -std=c++14

template <class T>
class RefPtr {
    T use_of_t;
};

template <typename U>
class UsesRefPtrWithAliasedTypeParam {
    using V = U;
    RefPtr<V> member;
};
