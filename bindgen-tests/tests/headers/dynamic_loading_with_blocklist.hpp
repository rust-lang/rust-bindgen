// bindgen-flags: --dynamic-loading TestLib --blocklist-function baz

class X {
  int _x;

 public:
  X(int x);

  void some_function();
  void some_other_function();
};

int foo(void *x);
int bar(void *x);
int baz(void *x);
