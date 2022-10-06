// bindgen-flags: --ctypes-prefix "libc::foo" --use-core --raw-line "#![no_std]" --raw-line "mod libc { pub mod foo { pub type c_int = i32; pub enum c_void {} } }" --rustified-enum ".*"
struct foo {
  int a, b;
  void* bar;
};
