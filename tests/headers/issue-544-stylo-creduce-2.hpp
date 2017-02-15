// bindgen-flags: -- -std=c++14

template <typename T>
struct Foo {
    template <typename> using FirstAlias = typename T::Associated;
    template <typename U> using SecondAlias = Foo<FirstAlias<U>>;
    SecondAlias<int> member;
};
