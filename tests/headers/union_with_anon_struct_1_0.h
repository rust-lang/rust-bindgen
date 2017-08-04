// bindgen-flags: --rust-target 1.0

union foo {
    struct {
        unsigned int a;
        unsigned int b;
    } bar;
};
