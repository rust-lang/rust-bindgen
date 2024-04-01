#![allow(dead_code, non_snake_case, non_camel_case_types, non_upper_case_globals)]
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct NoDefault {
    pub i: ::std::os::raw::c_int,
}
const _: () = {
    ["Size of NoDefault"][::std::mem::size_of::<NoDefault>() - 4usize];
    ["Alignment of NoDefault"][::std::mem::align_of::<NoDefault>() - 4usize];
    ["Offset of field: NoDefault::i"][::std::mem::offset_of!(NoDefault, i) - 0usize];
};
