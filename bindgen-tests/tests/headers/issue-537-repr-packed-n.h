// bindgen-flags: --raw-line '#![cfg(feature = "nightly")]' --rust-target 1.33

/// This should not be opaque; we can see the attributes and can pack the
/// struct.
struct AlignedToOne {
    int i;
} __attribute__ ((packed,aligned(1)));

/// This should be packed because Rust 1.33 has `#[repr(packed(N))]`.
struct AlignedToTwo {
    int i;
} __attribute__ ((packed,aligned(2)));

#pragma pack(1)

/// This should not be opaque because although `libclang` doesn't give us the
/// `#pragma pack(1)`, we can detect that alignment is 1 and add
/// `#[repr(packed)]` to the struct ourselves.
struct PackedToOne {
    int x;
    int y;
};

#pragma pack()

#pragma pack(2)

/// This should be packed because Rust 1.33 has `#[repr(packed(N))]`.
struct PackedToTwo {
    int x;
    int y;
};

#pragma pack()
