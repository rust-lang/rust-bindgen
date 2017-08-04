// bindgen-flags: --rust-target 1.0

union foo {
    int a;
    struct {
        int b : 7;
        int c : 25;
    };
};
