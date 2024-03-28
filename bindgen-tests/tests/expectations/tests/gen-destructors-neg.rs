#![allow(dead_code, non_snake_case, non_camel_case_types, non_upper_case_globals)]
#[repr(C)]
#[derive(Debug, Default)]
pub struct Foo {
    pub bar: ::std::os::raw::c_int,
}
const _: () = {
    assert!(::std::mem::size_of::<Foo>() == 4usize, "Size of Foo");
    assert!(::std::mem::align_of::<Foo>() == 4usize, "Alignment of Foo");
    assert!(::std::mem::offset_of!(Foo, bar) == 0usize, "Offset of field: Foo::bar");
};
