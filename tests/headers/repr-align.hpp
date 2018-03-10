// bindgen-flags: --rust-target 1.24 -- -std=c++11

struct alignas(8) a {
  int b;
  int c;
};
