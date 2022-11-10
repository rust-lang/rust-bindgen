typedef const struct foo {
    char inner;
} *foo;

typedef struct bar {
    char inner;
} *bar;

typedef struct baz *baz;

typedef union cat {
    int standard_issue;
} *cat;

typedef enum mad { scientist } *mad;

void takes_foo_ptr(foo);
void takes_foo_struct(struct foo);

void takes_bar_ptr(bar);
void takes_bar_struct(struct bar);

void takes_baz_ptr(baz);
void takes_baz_struct(struct baz);

void takes_cat_ptr(cat);
void takes_cat_union(union cat);

void takes_mad_ptr(mad);
void takes_mad_enum(enum mad);
