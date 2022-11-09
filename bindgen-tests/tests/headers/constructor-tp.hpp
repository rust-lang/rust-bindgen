
template<typename T>
class Foo {
public:
  Foo();

  void doBaz();
};

template<typename T>
void
Foo<T>::doBaz() {
}

class Bar {
public:
  Bar();
};

template<typename T>
Foo<T>::Foo() {
}

Bar::Bar() {
}
