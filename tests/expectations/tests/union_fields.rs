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
    fn test_field_mInt() {
        assert_eq!(
            unsafe {
                let uninit = ::std::mem::MaybeUninit::<nsStyleUnion>::uninit();
                let ptr = uninit.as_ptr();
                ::std::ptr::addr_of!((*ptr).mInt) as usize - ptr as usize
            },
            0usize,
            concat!(
                "Offset of field: ",
                stringify!(nsStyleUnion),
                "::",
                stringify!(mInt)
            )
        );
    }
    test_field_mInt();
    fn test_field_mFloat() {
        assert_eq!(
            unsafe {
                let uninit = ::std::mem::MaybeUninit::<nsStyleUnion>::uninit();
                let ptr = uninit.as_ptr();
                ::std::ptr::addr_of!((*ptr).mFloat) as usize - ptr as usize
            },
            0usize,
            concat!(
                "Offset of field: ",
                stringify!(nsStyleUnion),
                "::",
                stringify!(mFloat)
            )
        );
    }
    test_field_mFloat();
    fn test_field_mPointer() {
        assert_eq!(
            unsafe {
                let uninit = ::std::mem::MaybeUninit::<nsStyleUnion>::uninit();
                let ptr = uninit.as_ptr();
                ::std::ptr::addr_of!((*ptr).mPointer) as usize - ptr as usize
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
    test_field_mPointer();
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
