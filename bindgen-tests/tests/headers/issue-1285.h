// bindgen-flags: --with-derive-hash --no-recursive-allowlist --allowlist-type "foo"

struct foo {
    union {
        unsigned int a;
        unsigned short b;
    } bar;
};