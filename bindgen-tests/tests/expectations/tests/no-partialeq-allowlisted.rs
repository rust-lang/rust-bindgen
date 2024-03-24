#![allow(dead_code, non_snake_case, non_camel_case_types, non_upper_case_globals)]
#[repr(C)]
#[derive(Debug, Default, Copy, Clone)]
pub struct NoPartialEq {
    pub i: ::std::os::raw::c_int,
}
const _: () = {
    ["Size of NoPartialEq"][::std::mem::size_of::<NoPartialEq>() - 4usize];
    ["Alignment of NoPartialEq"][::std::mem::align_of::<NoPartialEq>() - 4usize];
    ["Offset of field: NoPartialEq::i"][::std::mem::offset_of!(NoPartialEq, i) - 0usize];
};
