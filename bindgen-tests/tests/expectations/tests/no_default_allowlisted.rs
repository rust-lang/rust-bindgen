#![allow(dead_code, non_snake_case, non_camel_case_types, non_upper_case_globals)]
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct NoDefault {
    pub i: ::std::os::raw::c_int,
}
const _: () = {
    assert!(::std::mem::size_of::<NoDefault>() == 4usize, "Size of NoDefault");
    assert!(::std::mem::align_of::<NoDefault>() == 4usize, "Alignment of NoDefault");
    assert!(
        ::std::mem::offset_of!(NoDefault, i) == 0usize,
        "Offset of field: NoDefault::i",
    );
};
