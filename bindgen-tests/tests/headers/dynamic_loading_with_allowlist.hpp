// bindgen-flags: --dynamic-loading TestLib --allowlist-function baz --allowlist-function foo --allowlist-function bazz

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
int bazz(int, ...);
