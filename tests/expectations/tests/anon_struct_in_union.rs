#![allow(
    dead_code,
    non_snake_case,
    non_camel_case_types,
    non_upper_case_globals
)]

#[repr(C)]
#[derive(Copy, Clone)]
pub struct s {
    pub u: s__bindgen_ty_1,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub union s__bindgen_ty_1 {
    pub field: s__bindgen_ty_1_inner,
}
#[repr(C)]
#[derive(Debug, Default, Copy, Clone, Hash, PartialEq, Eq)]
pub struct s__bindgen_ty_1_inner {
    pub b: ::std::os::raw::c_int,
}
#[test]
fn bindgen_test_layout_s__bindgen_ty_1_inner() {
    assert_eq!(
        ::std::mem::size_of::<s__bindgen_ty_1_inner>(),
        4usize,
        concat!("Size of: ", stringify!(s__bindgen_ty_1_inner))
    );
    assert_eq!(
        ::std::mem::align_of::<s__bindgen_ty_1_inner>(),
        4usize,
        concat!("Alignment of ", stringify!(s__bindgen_ty_1_inner))
    );
    fn test_field_b() {
        assert_eq!(
            unsafe {
                let uninit =
                    ::std::mem::MaybeUninit::<s__bindgen_ty_1_inner>::uninit();
                let ptr = uninit.as_ptr();
                ::std::ptr::addr_of!((*ptr).b) as usize - ptr as usize
            },
            0usize,
            concat!(
                "Offset of field: ",
                stringify!(s__bindgen_ty_1_inner),
                "::",
                stringify!(b)
            )
        );
    }
    test_field_b();
}
#[test]
fn bindgen_test_layout_s__bindgen_ty_1() {
    assert_eq!(
        ::std::mem::size_of::<s__bindgen_ty_1>(),
        4usize,
        concat!("Size of: ", stringify!(s__bindgen_ty_1))
    );
    assert_eq!(
        ::std::mem::align_of::<s__bindgen_ty_1>(),
        4usize,
        concat!("Alignment of ", stringify!(s__bindgen_ty_1))
    );
    fn test_field_field() {
        assert_eq!(
            unsafe {
                let uninit =
                    ::std::mem::MaybeUninit::<s__bindgen_ty_1>::uninit();
                let ptr = uninit.as_ptr();
                ::std::ptr::addr_of!((*ptr).field) as usize - ptr as usize
            },
            0usize,
            concat!(
                "Offset of field: ",
                stringify!(s__bindgen_ty_1),
                "::",
                stringify!(field)
            )
        );
    }
    test_field_field();
}
impl Default for s__bindgen_ty_1 {
    fn default() -> Self {
        let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
#[test]
fn bindgen_test_layout_s() {
    assert_eq!(
        ::std::mem::size_of::<s>(),
        4usize,
        concat!("Size of: ", stringify!(s))
    );
    assert_eq!(
        ::std::mem::align_of::<s>(),
        4usize,
        concat!("Alignment of ", stringify!(s))
    );
    fn test_field_u() {
        assert_eq!(
            unsafe {
                let uninit = ::std::mem::MaybeUninit::<s>::uninit();
                let ptr = uninit.as_ptr();
                ::std::ptr::addr_of!((*ptr).u) as usize - ptr as usize
            },
            0usize,
            concat!("Offset of field: ", stringify!(s), "::", stringify!(u))
        );
    }
    test_field_u();
}
impl Default for s {
    fn default() -> Self {
        let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
