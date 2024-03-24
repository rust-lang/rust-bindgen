#![allow(dead_code, non_snake_case, non_camel_case_types, non_upper_case_globals)]
#[repr(C)]
#[derive(Debug, Default, Copy, Clone)]
pub struct a {
    pub b: ::std::os::raw::c_int,
}
const _: () = {
    ["Size of a"][::std::mem::size_of::<a>() - 4usize];
    ["Alignment of a"][::std::mem::align_of::<a>() - 4usize];
    ["Offset of field: a::b"][::std::mem::offset_of!(a, b) - 0usize];
};
#[repr(C)]
#[derive(Debug, Default, Copy, Clone)]
pub struct c {
    pub d: ::std::os::raw::c_int,
}
const _: () = {
    ["Size of c"][::std::mem::size_of::<c>() - 4usize];
    ["Alignment of c"][::std::mem::align_of::<c>() - 4usize];
    ["Offset of field: c::d"][::std::mem::offset_of!(c, d) - 0usize];
};
