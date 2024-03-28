#![allow(dead_code, non_snake_case, non_camel_case_types, non_upper_case_globals)]
#[repr(C, packed(2))]
#[derive(Debug, Default, Copy, Clone)]
pub struct Packed {
    pub a: ::std::os::raw::c_char,
    pub b: ::std::os::raw::c_short,
    pub c: ::std::os::raw::c_char,
    pub d: ::std::os::raw::c_int,
}
const _: () = {
    assert!(::std::mem::size_of::<Packed>() == 10usize, "Size of Packed");
    assert!(::std::mem::align_of::<Packed>() == 2usize, "Alignment of Packed");
    assert!(::std::mem::offset_of!(Packed, a) == 0usize, "Offset of field: Packed::a");
    assert!(::std::mem::offset_of!(Packed, b) == 2usize, "Offset of field: Packed::b");
    assert!(::std::mem::offset_of!(Packed, c) == 4usize, "Offset of field: Packed::c");
    assert!(::std::mem::offset_of!(Packed, d) == 6usize, "Offset of field: Packed::d");
};
