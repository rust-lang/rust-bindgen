#![allow(dead_code, non_snake_case, non_camel_case_types, non_upper_case_globals)]
pub type Baz = ::std::os::raw::c_int;
#[repr(C)]
#[derive(Debug, Default, Copy, Clone)]
pub struct Foo {
    pub anon: Foo__bindgen_ty_1,
}
#[repr(C)]
#[derive(Debug, Default, Copy, Clone)]
pub struct Foo__bindgen_ty_1 {
    pub a: ::std::os::raw::c_int,
}
#[test]
fn bindgen_test_layout_Foo__bindgen_ty_1() {
    const UNINIT: ::std::mem::MaybeUninit<Foo__bindgen_ty_1> = ::std::mem::MaybeUninit::uninit();
    let ptr = UNINIT.as_ptr();
    assert_eq!(
        ::std::mem::size_of::<Foo__bindgen_ty_1>(),
        4usize,
        concat!("Size of: ", stringify!(Foo__bindgen_ty_1)),
    );
    assert_eq!(
        ::std::mem::align_of::<Foo__bindgen_ty_1>(),
        4usize,
        concat!("Alignment of ", stringify!(Foo__bindgen_ty_1)),
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).a) as usize - ptr as usize },
        0usize,
        concat!("Offset of field: ", stringify!(Foo__bindgen_ty_1), "::", stringify!(a)),
    );
}
#[test]
fn bindgen_test_layout_Foo() {
    const UNINIT: ::std::mem::MaybeUninit<Foo> = ::std::mem::MaybeUninit::uninit();
    let ptr = UNINIT.as_ptr();
    assert_eq!(
        ::std::mem::size_of::<Foo>(),
        4usize,
        concat!("Size of: ", stringify!(Foo)),
    );
    assert_eq!(
        ::std::mem::align_of::<Foo>(),
        4usize,
        concat!("Alignment of ", stringify!(Foo)),
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).anon) as usize - ptr as usize },
        0usize,
        concat!("Offset of field: ", stringify!(Foo), "::", stringify!(anon)),
    );
}
