struct foo;

typedef struct bar {
  struct foo {
    int foo;
  } u;
} bar_t;

struct baz {
  struct foo f;
};
