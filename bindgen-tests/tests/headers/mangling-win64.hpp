// bindgen-flags: -- --target=x86_64-pc-win32

extern "C" void foo();

struct Foo {
  static bool sBar;
};
