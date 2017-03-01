typedef void (*my_fun_t)(int, int, int, int,
                         int, int, int, int,
                         int, int, int, int,
                         int, int, int, int);

struct Foo {
  my_fun_t callback;
};
