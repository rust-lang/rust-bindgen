// bindgen-flags: --with-derive-hash --with-derive-partialeq
struct Test {
  int foo;
  float bar;
  enum { T_NONE };
};

typedef enum {
  Foo,
  Bar,
} Baz;
