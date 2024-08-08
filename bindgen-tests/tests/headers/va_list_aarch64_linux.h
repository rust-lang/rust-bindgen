// bindgen-flags: -- --target=aarch64-unknown-linux-gnu

typedef __builtin_va_list va_list;
int vprintf(const char* format, va_list vlist);
