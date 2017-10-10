// bindgen-flags: --rust-target nightly --raw-line '#![cfg(feature = "nightly")]' --raw-line '#![feature(abi_thiscall)]' -- --target=i686-pc-windows-msvc

class Foo {
  public:
    void test();
    int test2(int var);
};
