// bindgen-flags: --opaque-type "B" --whitelist-type "C"

class A;

class B {
  static A a;
};

class C {
  B b;
};
