// bindgen-flags: -- -std=c++14

template <typename T>
struct Foo {
    template <typename> using FirstAlias = typename T::Associated;
    template <typename U> using SecondAlias = Foo<FirstAlias<U>>;

#if 0
    // FIXME: This regressed sometime between libclang 9 and 16, though it
    // never quite worked properly so...
    SecondAlias<int> member;
#else
    SecondAlias<int>* member;
#endif
};
