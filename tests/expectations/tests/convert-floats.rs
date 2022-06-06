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
    fn test_field_bar() {
        assert_eq!(
            unsafe {
                let uninit = ::std::mem::MaybeUninit::<foo>::uninit();
                let ptr = uninit.as_ptr();
                ::std::ptr::addr_of!((*ptr).bar) as usize - ptr as usize
            },
            0usize,
            concat!(
                "Offset of field: ",
                stringify!(foo),
                "::",
                stringify!(bar)
            )
        );
    }
    test_field_bar();
    fn test_field_baz() {
        assert_eq!(
            unsafe {
                let uninit = ::std::mem::MaybeUninit::<foo>::uninit();
                let ptr = uninit.as_ptr();
                ::std::ptr::addr_of!((*ptr).baz) as usize - ptr as usize
            },
            4usize,
            concat!(
                "Offset of field: ",
                stringify!(foo),
                "::",
                stringify!(baz)
            )
        );
    }
    test_field_baz();
    fn test_field_bazz() {
        assert_eq!(
            unsafe {
                let uninit = ::std::mem::MaybeUninit::<foo>::uninit();
                let ptr = uninit.as_ptr();
                ::std::ptr::addr_of!((*ptr).bazz) as usize - ptr as usize
            },
            8usize,
            concat!(
                "Offset of field: ",
                stringify!(foo),
                "::",
                stringify!(bazz)
            )
        );
    }
    test_field_bazz();
    fn test_field_bazzz() {
        assert_eq!(
            unsafe {
                let uninit = ::std::mem::MaybeUninit::<foo>::uninit();
                let ptr = uninit.as_ptr();
                ::std::ptr::addr_of!((*ptr).bazzz) as usize - ptr as usize
            },
            16usize,
            concat!(
                "Offset of field: ",
                stringify!(foo),
                "::",
                stringify!(bazzz)
            )
        );
    }
    test_field_bazzz();
    fn test_field_complexFloat() {
        assert_eq!(
            unsafe {
                let uninit = ::std::mem::MaybeUninit::<foo>::uninit();
                let ptr = uninit.as_ptr();
                ::std::ptr::addr_of!((*ptr).complexFloat) as usize -
                    ptr as usize
            },
            24usize,
            concat!(
                "Offset of field: ",
                stringify!(foo),
                "::",
                stringify!(complexFloat)
            )
        );
    }
    test_field_complexFloat();
    fn test_field_complexDouble() {
        assert_eq!(
            unsafe {
                let uninit = ::std::mem::MaybeUninit::<foo>::uninit();
                let ptr = uninit.as_ptr();
                ::std::ptr::addr_of!((*ptr).complexDouble) as usize -
                    ptr as usize
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
    test_field_complexDouble();
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
