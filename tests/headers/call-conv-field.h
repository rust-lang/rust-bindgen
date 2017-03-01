// bindgen-flags: -- -target i686-pc-win32

struct JNINativeInterface_ {
  int (__stdcall *GetVersion)(void *env);
};

__stdcall void bar();
