#![allow(dead_code, non_snake_case, non_camel_case_types, non_upper_case_globals)]
#[repr(C)]
#[derive(Debug, Default, Copy, Clone)]
pub struct Foo {
    pub _address: u8,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of Foo"][::std::mem::size_of::<Foo>() - 1usize];
    ["Alignment of Foo"][::std::mem::align_of::<Foo>() - 1usize];
};
unsafe extern "C" {
    #[link_name = "\u{1}_ZN3FooC1Ev"]
    pub fn Foo_Foo(this: *mut Foo);
}
impl Foo {
    #[inline]
    pub unsafe fn create() -> Self {
        let mut __bindgen_tmp = ::std::mem::MaybeUninit::uninit();
        Foo_Foo(__bindgen_tmp.as_mut_ptr());
        __bindgen_tmp.assume_init()
    }
}
