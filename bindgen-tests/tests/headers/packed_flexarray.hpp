// bindgen-flags: --rust-target nightly --flexarray-dst --raw-line '#![cfg(feature = "nightly")]' --raw-line '#![feature(ptr_metadata, layout_for_ptr)]'

#pragma pack(1)

typedef struct PackedTest
{
    short int Head;
    long int Tail[];
}
PackedTest;

#pragma pack()

typedef struct UnpackedTest
{
    short int Head;
    long int Tail[];
}
UnpackedTest;
