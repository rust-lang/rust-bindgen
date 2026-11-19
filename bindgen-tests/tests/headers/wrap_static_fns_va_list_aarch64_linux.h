// bindgen-flags: --wrap-static-fns -- --target=aarch64-unknown-linux-gnu
// bindgen-parse-callbacks: wrap-as-variadic-fn

static void takes_va_list(__builtin_va_list va);

static inline int takes_int_and_va_list(int i, __builtin_va_list va) {
    return i;
}
