#![allow(dead_code, non_snake_case, non_camel_case_types, non_upper_case_globals)]
/** This should not be opaque; we can see the attributes and can pack the
 struct.*/
#[repr(C, packed)]
#[derive(Debug, Default, Copy, Clone)]
pub struct AlignedToOne {
    pub i: ::std::os::raw::c_int,
}
const _: () = {
    assert!(::std::mem::size_of::<AlignedToOne>() == 4usize, "Size of AlignedToOne");
    assert!(
        ::std::mem::align_of::<AlignedToOne>() == 1usize,
        "Alignment of AlignedToOne",
    );
    assert!(
        ::std::mem::offset_of!(AlignedToOne, i) == 0usize,
        "Offset of field: AlignedToOne::i",
    );
};
/** This should be opaque because although we can see the attributes, Rust before
 1.33 doesn't have `#[repr(packed(N))]`.*/
#[repr(C, packed(2))]
#[derive(Debug, Default, Copy, Clone)]
pub struct AlignedToTwo {
    pub i: ::std::os::raw::c_int,
}
const _: () = {
    assert!(::std::mem::size_of::<AlignedToTwo>() == 4usize, "Size of AlignedToTwo");
    assert!(
        ::std::mem::align_of::<AlignedToTwo>() == 2usize,
        "Alignment of AlignedToTwo",
    );
    assert!(
        ::std::mem::offset_of!(AlignedToTwo, i) == 0usize,
        "Offset of field: AlignedToTwo::i",
    );
};
/** This should not be opaque because although `libclang` doesn't give us the
 `#pragma pack(1)`, we can detect that alignment is 1 and add
 `#[repr(packed)]` to the struct ourselves.*/
#[repr(C, packed)]
#[derive(Debug, Default, Copy, Clone)]
pub struct PackedToOne {
    pub x: ::std::os::raw::c_int,
    pub y: ::std::os::raw::c_int,
}
const _: () = {
    assert!(::std::mem::size_of::<PackedToOne>() == 8usize, "Size of PackedToOne");
    assert!(::std::mem::align_of::<PackedToOne>() == 1usize, "Alignment of PackedToOne");
    assert!(
        ::std::mem::offset_of!(PackedToOne, x) == 0usize,
        "Offset of field: PackedToOne::x",
    );
    assert!(
        ::std::mem::offset_of!(PackedToOne, y) == 4usize,
        "Offset of field: PackedToOne::y",
    );
};
/** In this case, even if we can detect the weird alignment triggered by
 `#pragma pack(2)`, we can't do anything about it because Rust before 1.33
 doesn't have `#[repr(packed(N))]`. Therefore, we must make it opaque.*/
#[repr(C, packed(2))]
#[derive(Debug, Default, Copy, Clone)]
pub struct PackedToTwo {
    pub x: ::std::os::raw::c_int,
    pub y: ::std::os::raw::c_int,
}
const _: () = {
    assert!(::std::mem::size_of::<PackedToTwo>() == 8usize, "Size of PackedToTwo");
    assert!(::std::mem::align_of::<PackedToTwo>() == 2usize, "Alignment of PackedToTwo");
    assert!(
        ::std::mem::offset_of!(PackedToTwo, x) == 0usize,
        "Offset of field: PackedToTwo::x",
    );
    assert!(
        ::std::mem::offset_of!(PackedToTwo, y) == 4usize,
        "Offset of field: PackedToTwo::y",
    );
};
