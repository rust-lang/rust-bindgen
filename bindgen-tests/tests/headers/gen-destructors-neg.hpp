// bindgen-flags: --generate types,functions
//
// NB: This is intended to _not_ generate destructors.

class Foo {
  int bar;
 public:
  ~Foo();
};
