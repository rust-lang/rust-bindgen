// bindgen-flags: --use-core --raw-line '#![cfg(not(target_os="windows"))] extern crate core;' --with-derive-hash --with-derive-partialeq --with-derive-eq

struct foo {
  int a, b;
  void* bar;
};

union {
  int bar;
  long baz;
} bazz;

typedef void (*fooFunction)(int bar);
