#![allow(dead_code, non_snake_case, non_camel_case_types, non_upper_case_globals)]
#[repr(C)]
#[derive(Debug, Default, Copy, Clone)]
pub struct B {
    pub _address: u8,
}
#[repr(C)]
#[derive(Debug, Default, Copy, Clone)]
pub struct C {
    pub _address: u8,
}
pub type C_U = B;
#[repr(C)]
#[derive(Debug, Default, Copy, Clone)]
pub struct A {
    pub u: u8,
}
#[test]
fn bindgen_test_layout_A() {
    const UNINIT: ::std::mem::MaybeUninit<A> = ::std::mem::MaybeUninit::uninit();
    let ptr = UNINIT.as_ptr();
    assert_eq!(::std::mem::size_of::<A>(), 1usize, concat!("Size of: ", stringify!(A)));
    assert_eq!(
        ::std::mem::align_of::<A>(),
        1usize,
        concat!("Alignment of ", stringify!(A)),
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).u) as usize - ptr as usize },
        0usize,
        concat!("Offset of field: ", stringify!(A), "::", stringify!(u)),
    );
}
#[test]
fn __bindgen_test_layout_C_open0_A_close0_instantiation() {
    assert_eq!(
        ::std::mem::size_of::<C>(),
        1usize,
        concat!("Size of template specialization: ", stringify!(C)),
    );
    assert_eq!(
        ::std::mem::align_of::<C>(),
        1usize,
        concat!("Alignment of template specialization: ", stringify!(C)),
    );
}
