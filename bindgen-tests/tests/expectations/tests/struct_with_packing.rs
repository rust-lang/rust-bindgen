#![allow(dead_code, non_snake_case, non_camel_case_types, non_upper_case_globals)]
#[repr(C, packed)]
#[derive(Debug, Default, Copy, Clone, Hash, PartialEq, Eq)]
pub struct a {
    pub b: ::std::os::raw::c_char,
    pub c: ::std::os::raw::c_short,
}
const _: () = {
    assert!(::std::mem::size_of::<a>() == 3usize, "Size of a");
    assert!(::std::mem::align_of::<a>() == 1usize, "Alignment of a");
    assert!(::std::mem::offset_of!(a, b) == 0usize, "Offset of field: a::b");
    assert!(::std::mem::offset_of!(a, c) == 1usize, "Offset of field: a::c");
};
