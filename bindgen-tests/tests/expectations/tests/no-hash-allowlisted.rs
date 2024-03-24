#![allow(dead_code, non_snake_case, non_camel_case_types, non_upper_case_globals)]
#[repr(C)]
#[derive(Debug, Default, Copy, Clone)]
pub struct NoHash {
    pub i: ::std::os::raw::c_int,
}
const _: () = {
    assert!(::std::mem::size_of::<NoHash>() == 4usize, "Size of NoHash");
    assert!(::std::mem::align_of::<NoHash>() == 4usize, "Alignment of NoHash");
    assert!(::std::mem::offset_of!(NoHash, i) == 0usize, "Offset of field: NoHash::i");
};
