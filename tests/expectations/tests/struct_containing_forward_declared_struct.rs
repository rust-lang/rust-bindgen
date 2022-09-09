#![allow(
    dead_code,
    non_snake_case,
    non_camel_case_types,
    non_upper_case_globals
)]

#[repr(C)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct a {
    pub val_a: *mut b,
}
#[test]
fn bindgen_test_layout_a() {
    const UNINIT: ::std::mem::MaybeUninit<a> =
        ::std::mem::MaybeUninit::uninit();
    let ptr = UNINIT.as_ptr();
    assert_eq!(
        ::std::mem::size_of::<a>(),
        8usize,
        concat!("Size of: ", stringify!(a))
    );
    assert_eq!(
        ::std::mem::align_of::<a>(),
        8usize,
        concat!("Alignment of ", stringify!(a))
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).val_a) as usize - ptr as usize },
        0usize,
        concat!("Offset of field: ", stringify!(a), "::", stringify!(val_a))
    );
}
impl Default for a {
    fn default() -> Self {
        let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
#[repr(C)]
#[derive(Debug, Default, Copy, Clone, Hash, PartialEq, Eq)]
pub struct b {
    pub val_b: ::std::os::raw::c_int,
}
#[test]
fn bindgen_test_layout_b() {
    const UNINIT: ::std::mem::MaybeUninit<b> =
        ::std::mem::MaybeUninit::uninit();
    let ptr = UNINIT.as_ptr();
    assert_eq!(
        ::std::mem::size_of::<b>(),
        4usize,
        concat!("Size of: ", stringify!(b))
    );
    assert_eq!(
        ::std::mem::align_of::<b>(),
        4usize,
        concat!("Alignment of ", stringify!(b))
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).val_b) as usize - ptr as usize },
        0usize,
        concat!("Offset of field: ", stringify!(b), "::", stringify!(val_b))
    );
}
