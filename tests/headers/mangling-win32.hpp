// bindgen-flags: -- --target=i686-pc-win32

extern "C" void foo();

struct Foo {
  static bool sBar;
};
