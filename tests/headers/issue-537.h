/// This should not be opaque; we can see the attributes and can pack the
/// struct.
struct AlignedToOne {
    int i;
} __attribute__ ((packed,aligned(1)));

/// This should be opaque because although we can see the attributes, Rust
/// doesn't have `#[repr(packed = "N")]` yet.
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

/// In this case, even if we can detect the weird alignment triggered by
/// `#pragma pack(2)`, we can't do anything about it because Rust doesn't have
/// `#[repr(packed = "N")]`. Therefore, we must make it opaque.
struct PackedToTwo {
    int x;
    int y;
};

#pragma pack()
