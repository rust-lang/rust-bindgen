template <typename T, typename U = int>
struct Foo {
    T t;
    U u;
};

static Foo<bool> bar;
