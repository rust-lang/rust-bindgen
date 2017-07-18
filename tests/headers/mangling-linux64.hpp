// bindgen-flags: -- --target=x86_64-unknown-linux

extern "C" void foo();

struct Foo {
  static bool sBar;
};
