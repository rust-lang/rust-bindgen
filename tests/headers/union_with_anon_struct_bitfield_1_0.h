// bindgen-flags: --rust-target 1.0 --with-derive-hash

union foo {
    int a;
    struct {
        int b : 7;
        int c : 25;
    };
};
