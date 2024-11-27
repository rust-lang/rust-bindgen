// bindgen-flags: \-\-rust-target=1.33 -- --target=i686-pc-windows-msvc

class Foo {
  public:
    void test();
    int test2(int var);
};
