// bindgen-flags: --with-derive-hash --with-derive-partialeq --with-derive-eq

struct a;
class Bar {
  a *b;
};
struct c {
  union {};
};
struct a {
  c d;
};
