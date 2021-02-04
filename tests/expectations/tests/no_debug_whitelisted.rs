#![allow(
    dead_code,
    non_snake_case,
    non_camel_case_types,
    non_upper_case_globals
)]

#[repr(C)]
#[derive(Default, Copy, Clone)]
pub struct NoDebug {
    pub i: ::std::os::raw::c_int,
}
#[test]
fn bindgen_test_layout_NoDebug() {
    assert_eq!(
        ::std::mem::size_of::<NoDebug>(),
        4usize,
        concat!("Size of: ", stringify!(NoDebug))
    );
    assert_eq!(
        ::std::mem::align_of::<NoDebug>(),
        4usize,
        concat!("Alignment of ", stringify!(NoDebug))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<NoDebug>())).i as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(NoDebug),
            "::",
            stringify!(i)
        )
    );
}
