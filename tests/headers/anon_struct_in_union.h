// bindgen-flags: --with-derive-hash
struct s {
  union {
    struct inner {
      int b;
    } field;
  } u;
};
