// bindgen-flags: --use-core --raw-line "extern crate core;" --with-derive-hash --with-derive-partialeq

struct foo {
  int a, b;
  void* bar;
};

union {
  int bar;
  long baz;
} bazz;

typedef void (*fooFunction)(int bar);
