
template<typename T>
class Foo {
public:
  Foo();
};

class Bar {
public:
  Bar();
};

template<typename T>
Foo<T>::Foo() {
}

inline
Bar::Bar() {
}
