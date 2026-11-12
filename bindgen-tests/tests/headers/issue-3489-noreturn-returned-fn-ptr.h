// bindgen-flags: --enable-function-attribute-detection -- -std=gnu11

typedef void (__attribute__((noreturn)) *noreturn_fn_ptr)(int);

void (__attribute__((noreturn)) *take(int t))(int);
void (__attribute__((noreturn)) *take_noreturn(int t))(int) __attribute__((noreturn));
noreturn_fn_ptr take_alias(int t);
noreturn_fn_ptr take_alias_noreturn(int t) __attribute__((noreturn));
void plain(int t) __attribute__((noreturn));
