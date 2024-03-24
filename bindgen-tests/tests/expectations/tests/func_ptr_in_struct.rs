#![allow(dead_code, non_snake_case, non_camel_case_types, non_upper_case_globals)]
#[repr(i32)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum baz {
    __bindgen_cannot_repr_c_on_empty_enum = 0,
}
#[repr(C)]
#[derive(Debug, Default, Copy, Clone, Hash, PartialEq, Eq)]
pub struct Foo {
    pub bar: ::std::option::Option<
        unsafe extern "C" fn(x: ::std::os::raw::c_int, y: ::std::os::raw::c_int) -> baz,
    >,
}
const _: () = {
    ["Size of Foo"][::std::mem::size_of::<Foo>() - 8usize];
    ["Alignment of Foo"][::std::mem::align_of::<Foo>() - 8usize];
    ["Offset of field: Foo::bar"][::std::mem::offset_of!(Foo, bar) - 0usize];
};
