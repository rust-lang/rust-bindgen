// bindgen-flags: --with-derive-hash --with-derive-partialeq --with-derive-eq --rustified-enum .*
struct Test {
  int foo;
  float bar;
  enum { T_NONE };
};

typedef enum {
  Foo,
  Bar,
} Baz;
