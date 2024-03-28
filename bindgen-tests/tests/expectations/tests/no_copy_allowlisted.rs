#![allow(dead_code, non_snake_case, non_camel_case_types, non_upper_case_globals)]
#[repr(C)]
#[derive(Debug, Default)]
pub struct NoCopy {
    pub i: ::std::os::raw::c_int,
}
const _: () = {
    assert!(::std::mem::size_of::<NoCopy>() == 4usize, "Size of NoCopy");
    assert!(::std::mem::align_of::<NoCopy>() == 4usize, "Alignment of NoCopy");
    assert!(::std::mem::offset_of!(NoCopy, i) == 0usize, "Offset of field: NoCopy::i");
};
