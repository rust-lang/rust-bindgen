// bindgen-flags: --rust-target 1.0 --with-derive-hash --with-derive-partialeq

union foo {
    int a;
    struct {
        int b : 7;
        int c : 25;
    };
};
