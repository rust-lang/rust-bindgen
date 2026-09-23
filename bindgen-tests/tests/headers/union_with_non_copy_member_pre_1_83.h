// bindgen-flags: --no-copy 'NonCopyType' --rust-target=1.82

struct NonCopyType {
    int foo;
};

union WithBindgenGeneratedWrapper {
    struct NonCopyType non_copy_type;
    int bar;
};
