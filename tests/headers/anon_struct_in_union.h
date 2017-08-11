// bindgen-flags: --with-derive-hash --with-derive-partialeq
struct s {
  union {
    struct inner {
      int b;
    } field;
  } u;
};
