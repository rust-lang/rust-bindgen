// bindgen-flags: --tuple-varargs-len 5

void foo(const char* fmt, ...);

struct Bar {
  void foo(const char* fmt, ...);
};
