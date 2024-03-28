#![allow(dead_code, non_snake_case, non_camel_case_types, non_upper_case_globals)]
#[derive(PartialEq, Copy, Clone, Hash, Debug, Default)]
#[repr(C, align(16))]
pub struct __BindgenLongDouble([u8; 16]);
#[repr(C)]
#[repr(align(16))]
#[derive(Debug, Default, Copy, Clone)]
pub struct foo {
    pub bar: __BindgenLongDouble,
}
#[test]
fn bindgen_test_layout_foo() {
    const UNINIT: ::std::mem::MaybeUninit<foo> = ::std::mem::MaybeUninit::uninit();
    let ptr = UNINIT.as_ptr();
    assert_eq!(
        ::std::mem::size_of::<foo>(),
        16usize,
        concat!("Size of: ", stringify!(foo)),
    );
    assert_eq!(
        ::std::mem::align_of::<foo>(),
        16usize,
        concat!("Alignment of ", stringify!(foo)),
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).bar) as usize - ptr as usize },
        0usize,
        concat!("Offset of field: ", stringify!(foo), "::", stringify!(bar)),
    );
}
