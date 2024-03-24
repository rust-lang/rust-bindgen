#![allow(dead_code, non_snake_case, non_camel_case_types, non_upper_case_globals)]
#[repr(C)]
#[derive(Debug, Default, Copy, Clone)]
pub struct Foo {
    pub _address: u8,
}
const _: () = {
    assert!(::std::mem::size_of::<Foo>() == 1usize, "Size of Foo");
    assert!(::std::mem::align_of::<Foo>() == 1usize, "Alignment of Foo");
};
extern "C" {
    #[link_name = "\u{1}_ZN3Foo3fooEv"]
    pub fn Foo_foo(this: *mut Foo) -> ::std::os::raw::c_int;
}
impl Foo {
    #[inline]
    pub unsafe fn foo(&mut self) -> ::std::os::raw::c_int {
        Foo_foo(self)
    }
}
