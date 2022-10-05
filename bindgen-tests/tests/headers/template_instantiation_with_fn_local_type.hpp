// bindgen-flags: -- -std=c++14
//
// https://github.com/rust-lang/rust-bindgen/issues/2036

template<typename T>
struct Foo {};
template<typename T>
Foo<T> foo{};

// Struct inside function
void f() {
  struct Bar {
    Bar() {}
  };
  foo<Bar>;
}

// Struct inside method
class Baz {
  void f() {
    struct Boo {
      Boo() {}
    };
    foo<Boo>;
  }
};

