#![allow(dead_code, non_snake_case, non_camel_case_types, non_upper_case_globals)]
#[repr(C)]
#[derive(Default)]
pub struct __IncompleteArrayField<T>(::std::marker::PhantomData<T>, [T; 0]);
impl<T> __IncompleteArrayField<T> {
    #[inline]
    pub const fn new() -> Self {
        __IncompleteArrayField(::std::marker::PhantomData, [])
    }
    #[inline]
    pub fn as_ptr(&self) -> *const T {
        self as *const _ as *const T
    }
    #[inline]
    pub fn as_mut_ptr(&mut self) -> *mut T {
        self as *mut _ as *mut T
    }
    #[inline]
    pub unsafe fn as_slice(&self, len: usize) -> &[T] {
        ::std::slice::from_raw_parts(self.as_ptr(), len)
    }
    #[inline]
    pub unsafe fn as_mut_slice(&mut self, len: usize) -> &mut [T] {
        ::std::slice::from_raw_parts_mut(self.as_mut_ptr(), len)
    }
}
impl<T> ::std::fmt::Debug for __IncompleteArrayField<T> {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.write_str("__IncompleteArrayField")
    }
}
#[repr(C)]
#[derive(Debug, Default)]
pub struct test {
    pub a: ::std::os::raw::c_int,
    pub zero_length_array: __IncompleteArrayField<::std::os::raw::c_char>,
}
#[test]
fn bindgen_test_layout_test() {
    const UNINIT: ::std::mem::MaybeUninit<test> = ::std::mem::MaybeUninit::uninit();
    let ptr = UNINIT.as_ptr();
    assert_eq!(
        ::std::mem::size_of::<test>(),
        4usize,
        concat!("Size of: ", stringify!(test)),
    );
    assert_eq!(
        ::std::mem::align_of::<test>(),
        4usize,
        concat!("Alignment of ", stringify!(test)),
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).a) as usize - ptr as usize },
        0usize,
        concat!("Offset of field: ", stringify!(test), "::", stringify!(a)),
    );
    assert_eq!(
        unsafe {
            ::std::ptr::addr_of!((*ptr).zero_length_array) as usize - ptr as usize
        },
        4usize,
        concat!(
            "Offset of field: ",
            stringify!(test),
            "::",
            stringify!(zero_length_array),
        ),
    );
}
#[repr(C)]
#[derive(Debug, Default)]
pub struct test2 {
    pub a: ::std::os::raw::c_int,
    pub incomplete_array: __IncompleteArrayField<::std::os::raw::c_char>,
}
#[test]
fn bindgen_test_layout_test2() {
    const UNINIT: ::std::mem::MaybeUninit<test2> = ::std::mem::MaybeUninit::uninit();
    let ptr = UNINIT.as_ptr();
    assert_eq!(
        ::std::mem::size_of::<test2>(),
        4usize,
        concat!("Size of: ", stringify!(test2)),
    );
    assert_eq!(
        ::std::mem::align_of::<test2>(),
        4usize,
        concat!("Alignment of ", stringify!(test2)),
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).a) as usize - ptr as usize },
        0usize,
        concat!("Offset of field: ", stringify!(test2), "::", stringify!(a)),
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).incomplete_array) as usize - ptr as usize },
        4usize,
        concat!(
            "Offset of field: ",
            stringify!(test2),
            "::",
            stringify!(incomplete_array),
        ),
    );
}
#[repr(C)]
#[derive(Debug, Default)]
pub struct test3 {
    pub a: ::std::os::raw::c_int,
    pub zero_length_array: __IncompleteArrayField<::std::os::raw::c_char>,
    pub incomplete_array: __IncompleteArrayField<::std::os::raw::c_char>,
}
#[test]
fn bindgen_test_layout_test3() {
    const UNINIT: ::std::mem::MaybeUninit<test3> = ::std::mem::MaybeUninit::uninit();
    let ptr = UNINIT.as_ptr();
    assert_eq!(
        ::std::mem::size_of::<test3>(),
        4usize,
        concat!("Size of: ", stringify!(test3)),
    );
    assert_eq!(
        ::std::mem::align_of::<test3>(),
        4usize,
        concat!("Alignment of ", stringify!(test3)),
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).a) as usize - ptr as usize },
        0usize,
        concat!("Offset of field: ", stringify!(test3), "::", stringify!(a)),
    );
    assert_eq!(
        unsafe {
            ::std::ptr::addr_of!((*ptr).zero_length_array) as usize - ptr as usize
        },
        4usize,
        concat!(
            "Offset of field: ",
            stringify!(test3),
            "::",
            stringify!(zero_length_array),
        ),
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).incomplete_array) as usize - ptr as usize },
        4usize,
        concat!(
            "Offset of field: ",
            stringify!(test3),
            "::",
            stringify!(incomplete_array),
        ),
    );
}
