// bindgen-flags: -- --target=i586-unknown-linux

extern "C" void foo();

struct Foo {
  static bool sBar;
};
