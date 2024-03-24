#![allow(dead_code, non_snake_case, non_camel_case_types, non_upper_case_globals)]
#[repr(C)]
#[derive(Debug, Default, Copy, Clone)]
pub struct SizedIntegers {
    pub x: u8,
    pub y: u16,
    pub z: u32,
}
const _: () = {
    assert!(::std::mem::size_of::<SizedIntegers>() == 8usize, "Size of SizedIntegers");
    assert!(
        ::std::mem::align_of::<SizedIntegers>() == 4usize,
        "Alignment of SizedIntegers",
    );
    assert!(
        ::std::mem::offset_of!(SizedIntegers, x) == 0usize,
        "Offset of field: SizedIntegers::x",
    );
    assert!(
        ::std::mem::offset_of!(SizedIntegers, y) == 2usize,
        "Offset of field: SizedIntegers::y",
    );
    assert!(
        ::std::mem::offset_of!(SizedIntegers, z) == 4usize,
        "Offset of field: SizedIntegers::z",
    );
};
#[repr(C)]
#[derive(Debug, Default, Copy, Clone)]
pub struct StructWithBlocklistedFwdDecl {
    pub b: u8,
}
const _: () = {
    assert!(
        ::std::mem::size_of::<StructWithBlocklistedFwdDecl>() == 1usize,
        "Size of StructWithBlocklistedFwdDecl",
    );
    assert!(
        ::std::mem::align_of::<StructWithBlocklistedFwdDecl>() == 1usize,
        "Alignment of StructWithBlocklistedFwdDecl",
    );
    assert!(
        ::std::mem::offset_of!(StructWithBlocklistedFwdDecl, b) == 0usize,
        "Offset of field: StructWithBlocklistedFwdDecl::b",
    );
};
