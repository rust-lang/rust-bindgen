// bindgen-flags: --dont-wrap-fn-ptr-fields --no-default="foo"

typedef struct foo {
    int (*bar)();
} foo;
