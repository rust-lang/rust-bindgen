#![allow(dead_code, non_snake_case, non_camel_case_types, non_upper_case_globals)]
#[repr(C)]
#[derive(Debug, Default, Copy, Clone)]
pub struct a {
    pub b: ::std::os::raw::c_int,
}
const _: () = {
    assert!(::std::mem::size_of::<a>() == 4usize, "Size of a");
    assert!(::std::mem::align_of::<a>() == 4usize, "Alignment of a");
    assert!(::std::mem::offset_of!(a, b) == 0usize, "Offset of field: a::b");
};
#[repr(C)]
#[derive(Debug, Default, Copy, Clone)]
pub struct c {
    pub d: ::std::os::raw::c_int,
}
const _: () = {
    assert!(::std::mem::size_of::<c>() == 4usize, "Size of c");
    assert!(::std::mem::align_of::<c>() == 4usize, "Alignment of c");
    assert!(::std::mem::offset_of!(c, d) == 0usize, "Offset of field: c::d");
};
