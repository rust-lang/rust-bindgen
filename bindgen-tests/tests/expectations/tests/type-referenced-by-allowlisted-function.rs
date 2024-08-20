#![allow(dead_code, non_snake_case, non_camel_case_types, non_upper_case_globals)]
#[repr(C)]
#[derive(Debug, Default, Copy, Clone)]
pub struct dl_phdr_info {
    pub x: ::std::os::raw::c_int,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of dl_phdr_info"][::std::mem::size_of::<dl_phdr_info>() - 4usize];
    ["Alignment of dl_phdr_info"][::std::mem::align_of::<dl_phdr_info>() - 4usize];
    [
        "Offset of field: dl_phdr_info::x",
    ][::std::mem::offset_of!(dl_phdr_info, x) - 0usize];
};
extern "C" {
    pub fn dl_iterate_phdr(arg1: *mut dl_phdr_info) -> ::std::os::raw::c_int;
}
