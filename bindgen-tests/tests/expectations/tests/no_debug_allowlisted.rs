#![allow(dead_code, non_snake_case, non_camel_case_types, non_upper_case_globals)]
#[repr(C)]
#[derive(Default, Copy, Clone)]
pub struct NoDebug {
    pub i: ::std::os::raw::c_int,
}
const _: () = {
    assert!(::std::mem::size_of::<NoDebug>() == 4usize, "Size of NoDebug");
    assert!(::std::mem::align_of::<NoDebug>() == 4usize, "Alignment of NoDebug");
    assert!(::std::mem::offset_of!(NoDebug, i) == 0usize, "Offset of field: NoDebug::i");
};
