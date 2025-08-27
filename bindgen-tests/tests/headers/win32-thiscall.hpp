// bindgen-flags: --raw-line "#![cfg(not(test))]" -- --target=i686-pc-windows-msvc

class Foo {
  public:
    void test();
    int test2(int var);
};
