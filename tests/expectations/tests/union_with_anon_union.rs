#![allow(
    dead_code,
    non_snake_case,
    non_camel_case_types,
    non_upper_case_globals
)]

#[repr(C)]
#[derive(Copy, Clone)]
pub union foo {
    pub bar: foo__bindgen_ty_1,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub union foo__bindgen_ty_1 {
    pub a: ::std::os::raw::c_uint,
    pub b: ::std::os::raw::c_ushort,
}
#[test]
fn bindgen_test_layout_foo__bindgen_ty_1() {
    const UNINIT: ::std::mem::MaybeUninit<foo__bindgen_ty_1> =
        ::std::mem::MaybeUninit::uninit();
    let ptr = UNINIT.as_ptr();
    assert_eq!(
        ::std::mem::size_of::<foo__bindgen_ty_1>(),
        4usize,
        concat!("Size of: ", stringify!(foo__bindgen_ty_1))
    );
    assert_eq!(
        ::std::mem::align_of::<foo__bindgen_ty_1>(),
        4usize,
        concat!("Alignment of ", stringify!(foo__bindgen_ty_1))
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).a) as usize - ptr as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(foo__bindgen_ty_1),
            "::",
            stringify!(a)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).b) as usize - ptr as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(foo__bindgen_ty_1),
            "::",
            stringify!(b)
        )
    );
}
impl Default for foo__bindgen_ty_1 {
    fn default() -> Self {
        let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
#[test]
fn bindgen_test_layout_foo() {
    const UNINIT: ::std::mem::MaybeUninit<foo> =
        ::std::mem::MaybeUninit::uninit();
    let ptr = UNINIT.as_ptr();
    assert_eq!(
        ::std::mem::size_of::<foo>(),
        4usize,
        concat!("Size of: ", stringify!(foo))
    );
    assert_eq!(
        ::std::mem::align_of::<foo>(),
        4usize,
        concat!("Alignment of ", stringify!(foo))
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).bar) as usize - ptr as usize },
        0usize,
        concat!("Offset of field: ", stringify!(foo), "::", stringify!(bar))
    );
}
impl Default for foo {
    fn default() -> Self {
        let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
