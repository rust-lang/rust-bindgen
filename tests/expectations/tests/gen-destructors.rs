#![allow(
    dead_code,
    non_snake_case,
    non_camel_case_types,
    non_upper_case_globals
)]

#[repr(C)]
#[derive(Debug, Default)]
pub struct Foo {
    pub bar: ::std::os::raw::c_int,
}
#[test]
fn bindgen_test_layout_Foo() {
    const UNINIT: ::std::mem::MaybeUninit<Foo> =
        ::std::mem::MaybeUninit::uninit();
    let ptr = UNINIT.as_ptr();
    assert_eq!(
        ::std::mem::size_of::<Foo>(),
        4usize,
        concat!("Size of: ", stringify!(Foo))
    );
    assert_eq!(
        ::std::mem::align_of::<Foo>(),
        4usize,
        concat!("Alignment of ", stringify!(Foo))
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).bar) as usize - ptr as usize },
        0usize,
        concat!("Offset of field: ", stringify!(Foo), "::", stringify!(bar))
    );
}
extern "C" {
    #[link_name = "\u{1}_ZN3FooD1Ev"]
    pub fn Foo_Foo_destructor(this: *mut Foo);
}
impl Foo {
    #[inline]
    pub unsafe fn destruct(&mut self) {
        Foo_Foo_destructor(self)
    }
}
