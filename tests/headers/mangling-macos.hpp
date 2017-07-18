// bindgen-flags: -- --target=x86_64-apple-darwin

extern "C" void foo();

struct Foo {
  static bool sBar;
};
