// bindgen-flags: --dynamic-loading TestLib

int foo(void *x);

class A {
  int _x;

 public:
  A(int x);

  void some_function();
  void some_other_function();
};

void bar();
