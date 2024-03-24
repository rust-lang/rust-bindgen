#![allow(dead_code, non_snake_case, non_camel_case_types, non_upper_case_globals)]
#[repr(C)]
#[derive(Debug, Default, Copy, Clone)]
pub struct Foo {
    pub _address: u8,
}
const _: () = {
    ["Size of Foo"][::std::mem::size_of::<Foo>() - 1usize];
    ["Alignment of Foo"][::std::mem::align_of::<Foo>() - 1usize];
};
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
