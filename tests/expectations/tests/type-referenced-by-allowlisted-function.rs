#![allow(
    dead_code,
    non_snake_case,
    non_camel_case_types,
    non_upper_case_globals
)]

#[repr(C)]
#[derive(Debug, Default, Copy, Clone)]
pub struct dl_phdr_info {
    pub x: ::std::os::raw::c_int,
}
#[test]
fn bindgen_test_layout_dl_phdr_info() {
    assert_eq!(
        ::std::mem::size_of::<dl_phdr_info>(),
        4usize,
        concat!("Size of: ", stringify!(dl_phdr_info))
    );
    assert_eq!(
        ::std::mem::align_of::<dl_phdr_info>(),
        4usize,
        concat!("Alignment of ", stringify!(dl_phdr_info))
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<dl_phdr_info>())).x as *const _ as usize
        },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(dl_phdr_info),
            "::",
            stringify!(x)
        )
    );
}
extern "C" {
    pub fn dl_iterate_phdr(arg1: *mut dl_phdr_info) -> ::std::os::raw::c_int;
}
