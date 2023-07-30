// bindgen-flags: -- --target=x86_64-pc-windows-msvc

struct foo {
  float bar, baz;
  double bazz;
  long double* bazzz;
  float _Complex complexFloat;
  double _Complex complexDouble;
};
