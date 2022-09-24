// bindgen-flags: --generate-inline-functions

class Foo {
 public:
  static inline int bar() {
    return 42;
  }
};

inline int foo() {
  return 42;
}
