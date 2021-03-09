#![allow(
    dead_code,
    non_snake_case,
    non_camel_case_types,
    non_upper_case_globals
)]

#[repr(C)]
#[derive(Debug, Default, Copy, Clone)]
pub struct rlimit;

#[repr(C)]
#[derive(Debug, Default, Copy, Clone)]
pub struct my_rlimit_conf {
    pub core: rlimit,
    pub cpu: rlimit,
    pub data: rlimit,
    pub fsize: rlimit,
}
#[test]
fn bindgen_test_layout_my_rlimit_conf() {
    assert_eq!(
        ::std::mem::size_of::<my_rlimit_conf>(),
        0usize,
        concat!("Size of: ", stringify!(my_rlimit_conf))
    );
    assert_eq!(
        ::std::mem::align_of::<my_rlimit_conf>(),
        1usize,
        concat!("Alignment of ", stringify!(my_rlimit_conf))
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<my_rlimit_conf>())).core as *const _ as usize
        },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(my_rlimit_conf),
            "::",
            stringify!(core)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<my_rlimit_conf>())).cpu as *const _ as usize
        },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(my_rlimit_conf),
            "::",
            stringify!(cpu)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<my_rlimit_conf>())).data as *const _ as usize
        },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(my_rlimit_conf),
            "::",
            stringify!(data)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<my_rlimit_conf>())).fsize as *const _
                as usize
        },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(my_rlimit_conf),
            "::",
            stringify!(fsize)
        )
    );
}
