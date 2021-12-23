#![allow(
    dead_code,
    non_snake_case,
    non_camel_case_types,
    non_upper_case_globals
)]

#[derive(PartialEq, Copy, Clone, Hash, Debug, Default)]
#[repr(C)]
pub struct __BindgenComplex<T> {
    pub re: T,
    pub im: T,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct foo {
    pub bar: ::std::os::raw::c_float,
    pub baz: ::std::os::raw::c_float,
    pub bazz: ::std::os::raw::c_double,
    pub bazzz: *mut u128,
    pub complexFloat: __BindgenComplex<::std::os::raw::c_float>,
    pub complexDouble: __BindgenComplex<::std::os::raw::c_double>,
}
#[test]
fn bindgen_test_layout_foo() {
    assert_eq!(
        ::std::mem::size_of::<foo>(),
        48usize,
        concat!("Size of: ", stringify!(foo))
    );
    assert_eq!(
        ::std::mem::align_of::<foo>(),
        8usize,
        concat!("Alignment of ", stringify!(foo))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<foo>())).bar as *const _ as usize },
        0usize,
        concat!("Offset of field: ", stringify!(foo), "::", stringify!(bar))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<foo>())).baz as *const _ as usize },
        4usize,
        concat!("Offset of field: ", stringify!(foo), "::", stringify!(baz))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<foo>())).bazz as *const _ as usize },
        8usize,
        concat!("Offset of field: ", stringify!(foo), "::", stringify!(bazz))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<foo>())).bazzz as *const _ as usize },
        16usize,
        concat!(
            "Offset of field: ",
            stringify!(foo),
            "::",
            stringify!(bazzz)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<foo>())).complexFloat as *const _ as usize
        },
        24usize,
        concat!(
            "Offset of field: ",
            stringify!(foo),
            "::",
            stringify!(complexFloat)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<foo>())).complexDouble as *const _ as usize
        },
        32usize,
        concat!(
            "Offset of field: ",
            stringify!(foo),
            "::",
            stringify!(complexDouble)
        )
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
