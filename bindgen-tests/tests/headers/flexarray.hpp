// bindgen-flags: --rust-target nightly --flexarray-dst --raw-line '#![cfg(feature = "nightly")]' --raw-line '#![feature(ptr_metadata, layout_for_ptr)]'

struct flexarray {
    int count;
    int data[];
};

struct flexarray_zero {
    int count;
    int data[0];
};

template<typename T>
struct flexarray_template {
    int count;
    T data[];
};

struct flexarray_ref {
    flexarray *things;
};

struct flexarray_bogus_zero_fam {
    int count;
    int data1[0];
    char data2[];
};

struct flexarray_align {
    int count;
    int data[];
} __attribute__((aligned(128)));
