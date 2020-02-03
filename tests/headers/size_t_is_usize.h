// bindgen-flags: --size_t-is-usize

typedef unsigned long size_t;
typedef long ssize_t;

struct A {
  size_t len;
  ssize_t offset;
  struct A* next;
};
