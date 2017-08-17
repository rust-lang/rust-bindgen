// bindgen-flags: --opaque-type=.* --whitelist-function=foo  --with-derive-hash --with-derive-partialeq --with-derive-eq

class Container;

// The whitelist tracing should reach the Container type, even though it's
// marked as opaque.
void foo(Container* c);

template<typename T>
class Wat {
  T foo;
};

class OtherOpaque {
  int bar;
};

class Container {
  Wat<int> bar;
  OtherOpaque baz;
};
