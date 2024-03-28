#![allow(dead_code, non_snake_case, non_camel_case_types, non_upper_case_globals)]
#[derive(PartialEq, Copy, Clone, Hash, Debug, Default)]
#[repr(C)]
pub struct __BindgenComplex<T> {
    pub re: T,
    pub im: T,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct foo {
    pub bar: f32,
    pub baz: f32,
    pub bazz: f64,
    pub bazzz: *mut f64,
    pub complexFloat: __BindgenComplex<f32>,
    pub complexDouble: __BindgenComplex<f64>,
}
#[test]
fn bindgen_test_layout_foo() {
    const UNINIT: ::std::mem::MaybeUninit<foo> = ::std::mem::MaybeUninit::uninit();
    let ptr = UNINIT.as_ptr();
    assert_eq!(
        ::std::mem::size_of::<foo>(),
        48usize,
        concat!("Size of: ", stringify!(foo)),
    );
    assert_eq!(
        ::std::mem::align_of::<foo>(),
        8usize,
        concat!("Alignment of ", stringify!(foo)),
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).bar) as usize - ptr as usize },
        0usize,
        concat!("Offset of field: ", stringify!(foo), "::", stringify!(bar)),
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).baz) as usize - ptr as usize },
        4usize,
        concat!("Offset of field: ", stringify!(foo), "::", stringify!(baz)),
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).bazz) as usize - ptr as usize },
        8usize,
        concat!("Offset of field: ", stringify!(foo), "::", stringify!(bazz)),
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).bazzz) as usize - ptr as usize },
        16usize,
        concat!("Offset of field: ", stringify!(foo), "::", stringify!(bazzz)),
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).complexFloat) as usize - ptr as usize },
        24usize,
        concat!("Offset of field: ", stringify!(foo), "::", stringify!(complexFloat)),
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).complexDouble) as usize - ptr as usize },
        32usize,
        concat!("Offset of field: ", stringify!(foo), "::", stringify!(complexDouble)),
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
