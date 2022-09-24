// Issue #2085.

template<typename T>
struct Foo;

template<typename T, typename U>
struct Bar {};

template<typename T>
struct Bar<T, void> {
    using BarDependent = typename Foo<T>::Dependent;
    void method(const BarDependent &);
};

template<typename T>
void Bar<T, void>::method(const BarDependent &) {}
