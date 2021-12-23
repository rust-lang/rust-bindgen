#![allow(
    dead_code,
    non_snake_case,
    non_camel_case_types,
    non_upper_case_globals
)]

#[repr(C)]
#[derive(Copy, Clone)]
pub union nsStyleUnion {
    pub mInt: ::std::os::raw::c_int,
    pub mFloat: f32,
    pub mPointer: *mut ::std::os::raw::c_void,
}
#[test]
fn bindgen_test_layout_nsStyleUnion() {
    assert_eq!(
        ::std::mem::size_of::<nsStyleUnion>(),
        8usize,
        concat!("Size of: ", stringify!(nsStyleUnion))
    );
    assert_eq!(
        ::std::mem::align_of::<nsStyleUnion>(),
        8usize,
        concat!("Alignment of ", stringify!(nsStyleUnion))
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<nsStyleUnion>())).mInt as *const _ as usize
        },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(nsStyleUnion),
            "::",
            stringify!(mInt)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<nsStyleUnion>())).mFloat as *const _ as usize
        },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(nsStyleUnion),
            "::",
            stringify!(mFloat)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<nsStyleUnion>())).mPointer as *const _
                as usize
        },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(nsStyleUnion),
            "::",
            stringify!(mPointer)
        )
    );
}
impl Default for nsStyleUnion {
    fn default() -> Self {
        let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
