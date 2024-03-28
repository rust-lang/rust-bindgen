#![allow(dead_code, non_snake_case, non_camel_case_types, non_upper_case_globals)]
pub const FooDefault: u32 = 0;
#[repr(C)]
#[derive(Debug, Default, Copy, Clone)]
pub struct Foo {
    pub field: ::std::os::raw::c_int,
}
const _: () = {
    assert!(::std::mem::size_of::<Foo>() == 4usize, "Size of Foo");
    assert!(::std::mem::align_of::<Foo>() == 4usize, "Alignment of Foo");
    assert!(::std::mem::offset_of!(Foo, field) == 0usize, "Offset of field: Foo::field");
};
extern "C" {
    pub fn FooNew(value: ::std::os::raw::c_int) -> Foo;
}
