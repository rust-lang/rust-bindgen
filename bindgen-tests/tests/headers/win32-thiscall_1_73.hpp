// bindgen-flags: --rust-target=1.73 --raw-line '#![cfg(target = "i686-pc-windows-msvc")]' -- --target=i686-pc-windows-msvc

class Foo {
  public:
    void test();
    int test2(int var);
};
