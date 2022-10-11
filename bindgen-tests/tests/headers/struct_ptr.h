typedef const struct foo {
    char inner;
} *foo;

typedef struct bar {
    char inner;
} *bar;

void takes_foo_ptr(foo);
void takes_foo_struct(struct foo);
void takes_bar_ptr(bar);
void takes_bar_struct(struct bar);
