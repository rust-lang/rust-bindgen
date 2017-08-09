// bindgen-flags: --with-derive-hash
struct foo {
    struct {
        int a;
        int b;
    } *bar;
};
