// bindgen-flags: --dont-wrap-fn-ptr-fields

typedef struct foo {
    int (*bar)();
} foo;
