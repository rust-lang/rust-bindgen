template <typename T>
class Foo {
public:
    class InnerType {
        T t;
    };
};

typedef Foo<int>::InnerType Bar;

Bar func();