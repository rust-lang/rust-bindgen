// bindgen-flags: --enable-function-attribute-detection -- -std=gnu11

void (__attribute__((noreturn)) *take(int t))(int);
void plain(int t) __attribute__((noreturn));
