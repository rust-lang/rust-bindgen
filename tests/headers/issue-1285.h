// bindgen-flags: --with-derive-hash --no-recursive-whitelist --whitelist-type "foo"

struct foo {
    union {
        unsigned int a;
        unsigned short b;
    } bar;
};