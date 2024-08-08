// bindgen-flags: --rust-target 1.26

struct foo {
  long double bar;
};

void take_ld(long double ld);
