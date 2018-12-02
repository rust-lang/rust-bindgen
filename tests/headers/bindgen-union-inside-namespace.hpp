// bindgen-flags: --rust-target 1.0 --enable-cxx-namespaces

namespace foo {
  union Bar {
    int foo;
    int bar;
  };
}
