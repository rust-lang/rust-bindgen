// bindgen-flags: -- --target=i686-pc-win32

extern "C" void foo();

struct Foo {
  static bool sBar;
};

// Also test some x86 Windows specific calling conventions that have their own
// special mangling
extern "C" {
    int _fastcall fast_call_func_no_args();
    // This will result in a suffix with more than one character (i.e. `@12`)
    int _fastcall fast_call_func_many_args(int,int,int);
    int _stdcall std_call_func_no_args();
    // This will result in a suffix with more than one character (i.e. `@12`)
    int _stdcall std_call_func_many_args(int,int,int);
}
