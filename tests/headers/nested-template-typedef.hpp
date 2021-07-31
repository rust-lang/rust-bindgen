template<typename T>
class Foo {
public:
    template<typename U>
    struct Bar {
        typedef Foo<U> FooU;
    };
};
