// bindgen-flags: --rust-target 1.0 --with-derive-hash

struct s {
  union {
    struct inner {
      int b;
    } field;
  } u;
};
