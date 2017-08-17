// bindgen-flags: --opaque-type "B" --whitelist-type "C" --with-derive-hash --with-derive-partialeq --with-derive-eq

class A;

class B {
  static A a;
};

class C {
  B b;
};
