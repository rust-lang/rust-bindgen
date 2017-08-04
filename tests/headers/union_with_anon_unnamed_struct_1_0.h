// bindgen-flags: --rust-target 1.0

union pixel {
    unsigned int rgba;
    struct {
        unsigned char r;
        unsigned char g;
        unsigned char b;
        unsigned char a;
    };
};
