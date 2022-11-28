#![allow(
    dead_code,
    non_snake_case,
    non_camel_case_types,
    non_upper_case_globals
)]

#[repr(C)]
#[derive(Debug, Default, Copy, Clone)]
pub struct Foo {
    pub _address: u8,
}
#[test]
fn bindgen_test_layout_Foo() {
    assert_eq!(
        ::std::mem::size_of::<Foo>(),
        1usize,
        concat!("Size of: ", stringify!(Foo))
    );
    assert_eq!(
        ::std::mem::align_of::<Foo>(),
        1usize,
        concat!("Alignment of ", stringify!(Foo))
    );
}
extern "C" {
    #[link_name = "\u{1}_ZN3FooC1Ei"]
    pub fn Foo_Foo(this: *mut Foo, a: ::std::os::raw::c_int);
}
impl Foo {
    #[inline]
    pub unsafe fn new(a: ::std::os::raw::c_int) -> Self {
        let mut __bindgen_tmp = ::std::mem::MaybeUninit::uninit();
        Foo_Foo(__bindgen_tmp.as_mut_ptr(), a);
        __bindgen_tmp.assume_init()
    }
}
