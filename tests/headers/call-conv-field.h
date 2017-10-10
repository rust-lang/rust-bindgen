// bindgen-flags: -- --target=i686-pc-win32
// bindgen-unstable
// bindgen-generate-bindings-on-linux-only
//
// The linux-only thing is a hack around our lack of understanding when
// bindgen's target_os != the bindings' target_os :(
//
// https://github.com/rust-lang-nursery/rust-bindgen/issues/593

struct JNINativeInterface_ {
  int (__stdcall *GetVersion)(void *env);
  unsigned long long __hack; // A hack so the field alignment is the same than
                             // for 64-bit, where we run CI.
};

__stdcall void bar();
