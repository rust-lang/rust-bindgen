#![allow(
    dead_code,
    non_snake_case,
    non_camel_case_types,
    non_upper_case_globals
)]

#[repr(C)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct S {
    pub large_array: [::std::os::raw::c_char; 33usize],
}
#[test]
fn bindgen_test_layout_S() {
    const UNINIT: ::std::mem::MaybeUninit<S> =
        ::std::mem::MaybeUninit::uninit();
    let ptr = UNINIT.as_ptr();
    assert_eq!(
        ::std::mem::size_of::<S>(),
        33usize,
        concat!("Size of: ", stringify!(S))
    );
    assert_eq!(
        ::std::mem::align_of::<S>(),
        1usize,
        concat!("Alignment of ", stringify!(S))
    );
    assert_eq!(
        unsafe {
            ::std::ptr::addr_of!((*ptr).large_array) as usize - ptr as usize
        },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(S),
            "::",
            stringify!(large_array)
        )
    );
}
impl Default for S {
    fn default() -> Self {
        let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
#[repr(C)]
#[derive(Debug, Hash, PartialEq, Eq)]
pub struct ST<T> {
    pub large_array: [T; 33usize],
    pub _phantom_0: ::std::marker::PhantomData<::std::cell::UnsafeCell<T>>,
}
impl<T> Default for ST<T> {
    fn default() -> Self {
        let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
