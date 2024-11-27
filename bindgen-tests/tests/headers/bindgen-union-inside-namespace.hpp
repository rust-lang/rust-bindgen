// bindgen-flags: \-\-rust-target=1.33 --enable-cxx-namespaces

namespace foo {
  union Bar {
    int foo;
    int bar;
  };
}
