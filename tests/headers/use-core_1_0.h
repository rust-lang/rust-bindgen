// bindgen-flags: --rust-target 1.0 --use-core --raw-line "extern crate core;"

struct foo {
  int a, b;
  void* bar;
};

union {
  int bar;
  long baz;
} bazz;

typedef void (*fooFunction)(int bar);
