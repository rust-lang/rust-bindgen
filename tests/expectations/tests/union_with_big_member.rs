#![allow(
    dead_code,
    non_snake_case,
    non_camel_case_types,
    non_upper_case_globals
)]

#[repr(C)]
#[derive(Copy, Clone)]
pub union WithBigArray {
    pub a: ::std::os::raw::c_int,
    pub b: [::std::os::raw::c_int; 33usize],
}
#[test]
fn bindgen_test_layout_WithBigArray() {
    assert_eq!(
        ::std::mem::size_of::<WithBigArray>(),
        132usize,
        concat!("Size of: ", stringify!(WithBigArray))
    );
    assert_eq!(
        ::std::mem::align_of::<WithBigArray>(),
        4usize,
        concat!("Alignment of ", stringify!(WithBigArray))
    );
    fn test_field_a() {
        assert_eq!(
            unsafe {
                let uninit = ::std::mem::MaybeUninit::<WithBigArray>::uninit();
                let ptr = uninit.as_ptr();
                ::std::ptr::addr_of!((*ptr).a) as usize - ptr as usize
            },
            0usize,
            concat!(
                "Offset of field: ",
                stringify!(WithBigArray),
                "::",
                stringify!(a)
            )
        );
    }
    test_field_a();
    fn test_field_b() {
        assert_eq!(
            unsafe {
                let uninit = ::std::mem::MaybeUninit::<WithBigArray>::uninit();
                let ptr = uninit.as_ptr();
                ::std::ptr::addr_of!((*ptr).b) as usize - ptr as usize
            },
            0usize,
            concat!(
                "Offset of field: ",
                stringify!(WithBigArray),
                "::",
                stringify!(b)
            )
        );
    }
    test_field_b();
}
impl Default for WithBigArray {
    fn default() -> Self {
        let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
#[repr(C)]
#[derive(Copy, Clone)]
pub union WithBigArray2 {
    pub a: ::std::os::raw::c_int,
    pub b: [::std::os::raw::c_char; 33usize],
}
#[test]
fn bindgen_test_layout_WithBigArray2() {
    assert_eq!(
        ::std::mem::size_of::<WithBigArray2>(),
        36usize,
        concat!("Size of: ", stringify!(WithBigArray2))
    );
    assert_eq!(
        ::std::mem::align_of::<WithBigArray2>(),
        4usize,
        concat!("Alignment of ", stringify!(WithBigArray2))
    );
    fn test_field_a() {
        assert_eq!(
            unsafe {
                let uninit = ::std::mem::MaybeUninit::<WithBigArray2>::uninit();
                let ptr = uninit.as_ptr();
                ::std::ptr::addr_of!((*ptr).a) as usize - ptr as usize
            },
            0usize,
            concat!(
                "Offset of field: ",
                stringify!(WithBigArray2),
                "::",
                stringify!(a)
            )
        );
    }
    test_field_a();
    fn test_field_b() {
        assert_eq!(
            unsafe {
                let uninit = ::std::mem::MaybeUninit::<WithBigArray2>::uninit();
                let ptr = uninit.as_ptr();
                ::std::ptr::addr_of!((*ptr).b) as usize - ptr as usize
            },
            0usize,
            concat!(
                "Offset of field: ",
                stringify!(WithBigArray2),
                "::",
                stringify!(b)
            )
        );
    }
    test_field_b();
}
impl Default for WithBigArray2 {
    fn default() -> Self {
        let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
#[repr(C)]
#[derive(Copy, Clone)]
pub union WithBigMember {
    pub a: ::std::os::raw::c_int,
    pub b: WithBigArray,
}
#[test]
fn bindgen_test_layout_WithBigMember() {
    assert_eq!(
        ::std::mem::size_of::<WithBigMember>(),
        132usize,
        concat!("Size of: ", stringify!(WithBigMember))
    );
    assert_eq!(
        ::std::mem::align_of::<WithBigMember>(),
        4usize,
        concat!("Alignment of ", stringify!(WithBigMember))
    );
    fn test_field_a() {
        assert_eq!(
            unsafe {
                let uninit = ::std::mem::MaybeUninit::<WithBigMember>::uninit();
                let ptr = uninit.as_ptr();
                ::std::ptr::addr_of!((*ptr).a) as usize - ptr as usize
            },
            0usize,
            concat!(
                "Offset of field: ",
                stringify!(WithBigMember),
                "::",
                stringify!(a)
            )
        );
    }
    test_field_a();
    fn test_field_b() {
        assert_eq!(
            unsafe {
                let uninit = ::std::mem::MaybeUninit::<WithBigMember>::uninit();
                let ptr = uninit.as_ptr();
                ::std::ptr::addr_of!((*ptr).b) as usize - ptr as usize
            },
            0usize,
            concat!(
                "Offset of field: ",
                stringify!(WithBigMember),
                "::",
                stringify!(b)
            )
        );
    }
    test_field_b();
}
impl Default for WithBigMember {
    fn default() -> Self {
        let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
