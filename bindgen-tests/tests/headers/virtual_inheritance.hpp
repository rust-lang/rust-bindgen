// bindgen-flags: --no-layout-tests
// FIXME: Enable layout tests when #465 is fixed.
class A {
  int foo;
};

class B: public virtual A {
  int bar;
};

class C: public virtual A {
  int baz;
};

class D: public C, public B {
  int bazz;
};
