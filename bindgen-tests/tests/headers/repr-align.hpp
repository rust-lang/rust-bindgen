// bindgen-flags: --raw-line '#![cfg(feature = "nightly")]' \-\-rust-target=1.33 -- -std=c++11

struct alignas(8) a {
  int b;
  int c;
};

struct alignas(double) b {
  int b;
  int c;
};
