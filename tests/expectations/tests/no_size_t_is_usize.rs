#![allow(
    dead_code,
    non_snake_case,
    non_camel_case_types,
    non_upper_case_globals
)]

pub type size_t = ::std::os::raw::c_ulong;
pub type ssize_t = ::std::os::raw::c_long;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct A {
    pub len: size_t,
    pub offset: ssize_t,
    pub next: *mut A,
}
#[test]
fn bindgen_test_layout_A() {
    const UNINIT: ::std::mem::MaybeUninit<A> =
        ::std::mem::MaybeUninit::uninit();
    let ptr = UNINIT.as_ptr();
    assert_eq!(
        ::std::mem::size_of::<A>(),
        24usize,
        concat!("Size of: ", stringify!(A))
    );
    assert_eq!(
        ::std::mem::align_of::<A>(),
        8usize,
        concat!("Alignment of ", stringify!(A))
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).len) as usize - ptr as usize },
        0usize,
        concat!("Offset of field: ", stringify!(A), "::", stringify!(len))
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).offset) as usize - ptr as usize },
        8usize,
        concat!("Offset of field: ", stringify!(A), "::", stringify!(offset))
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).next) as usize - ptr as usize },
        16usize,
        concat!("Offset of field: ", stringify!(A), "::", stringify!(next))
    );
}
impl Default for A {
    fn default() -> Self {
        let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
