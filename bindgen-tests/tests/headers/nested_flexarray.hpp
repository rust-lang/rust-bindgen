// bindgen-flags: --rust-target nightly --flexarray-dst --raw-line '#![cfg(feature = "nightly")]' --raw-line '#![feature(ptr_metadata, layout_for_ptr)]'

// Test for nested flexible array members
struct Field {
    int count;
    int data[];  // FAM
};

struct Name {
    int id;
    struct Field field;  // Last field is a struct with FAM
};

#pragma pack(1)
struct NamePacked {
    int id;
    struct Field field;  // Last field is a struct with FAM, in a packed struct
};
#pragma pack()
