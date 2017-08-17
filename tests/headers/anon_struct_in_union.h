// bindgen-flags: --with-derive-hash --with-derive-partialeq --with-derive-eq
struct s {
  union {
    struct inner {
      int b;
    } field;
  } u;
};
