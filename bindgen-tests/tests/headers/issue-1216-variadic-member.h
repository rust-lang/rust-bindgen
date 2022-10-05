void f(int a, ...);
struct Foo {
  void (*f)(void *p, void *obj, int a, ...);
};
