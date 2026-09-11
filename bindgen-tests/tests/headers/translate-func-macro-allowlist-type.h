// bindgen-flags: --translate-function-macros --allowlist-type Foo

struct Foo {
  int x;
};
#define ADD(x, y) ((x) + (y))
