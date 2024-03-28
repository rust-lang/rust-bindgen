#![allow(dead_code, non_snake_case, non_camel_case_types, non_upper_case_globals)]
#[repr(C)]
#[derive(Debug, Default, Copy, Clone)]
pub struct dl_phdr_info {
    pub x: ::std::os::raw::c_int,
}
const _: () = {
    assert!(::std::mem::size_of::<dl_phdr_info>() == 4usize, "Size of dl_phdr_info");
    assert!(
        ::std::mem::align_of::<dl_phdr_info>() == 4usize,
        "Alignment of dl_phdr_info",
    );
    assert!(
        ::std::mem::offset_of!(dl_phdr_info, x) == 0usize,
        "Offset of field: dl_phdr_info::x",
    );
};
extern "C" {
    pub fn dl_iterate_phdr(arg1: *mut dl_phdr_info) -> ::std::os::raw::c_int;
}
