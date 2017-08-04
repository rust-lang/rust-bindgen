// bindgen-flags: --rust-target 1.0

struct s {
  union {
    struct inner {
      int b;
    } field;
  } u;
};
