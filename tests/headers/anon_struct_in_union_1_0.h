// bindgen-flags: --rust-target 1.0 --with-derive-hash --with-derive-partialeq

struct s {
  union {
    struct inner {
      int b;
    } field;
  } u;
};
