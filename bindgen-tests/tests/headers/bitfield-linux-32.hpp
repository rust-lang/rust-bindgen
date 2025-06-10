// bindgen-flags: --no-layout-tests -- --target=i586-unknown-linux

typedef unsigned long long uint64_t;

struct Test {
  uint64_t foo;
  uint64_t x : 56;
  uint64_t y : 8;
};
