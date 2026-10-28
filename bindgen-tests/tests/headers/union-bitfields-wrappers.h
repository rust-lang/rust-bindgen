// bindgen-flags: --bindgen-wrapper-union 'WithBindgenWrapper' --no-copy 'NonCopyType'

struct NonCopyType {
    unsigned char byte;
};

union WithBindgenWrapper {
    struct NonCopyType non_copy;
    unsigned int wide : 24;
    unsigned int narrow : 1;
};
