// bindgen-flags: --merge-impl-blocks --newtype-enum '.*' --enable-cxx-namespaces -- --target=x86_64-unknown-linux
int foo();
enum Foo {
  Bar = 1 << 1,
  Baz = 1 << 2,
  Duplicated = 1 << 2,
  Negative = -3,
};
int bar();

namespace ns {
    int foo();
    enum Bar {
      B0 = 1,
      B1 = B0 + 3,
      B2 = B0 + 2,
      B3 = B0 - 2,
    };
    int bar();
}
