#![allow(dead_code, non_snake_case, non_camel_case_types, non_upper_case_globals)]
#[repr(C)]
#[derive(Debug, Default)]
pub struct Foo {
    pub bar: ::std::os::raw::c_int,
}
const _: () = {
    ["Size of Foo"][::std::mem::size_of::<Foo>() - 4usize];
    ["Alignment of Foo"][::std::mem::align_of::<Foo>() - 4usize];
    ["Offset of field: Foo::bar"][::std::mem::offset_of!(Foo, bar) - 0usize];
};
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
