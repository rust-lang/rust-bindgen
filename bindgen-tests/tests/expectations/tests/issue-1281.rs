#![allow(dead_code, non_snake_case, non_camel_case_types, non_upper_case_globals)]
#[repr(C)]
#[derive(Debug, Default, Copy, Clone)]
pub struct bar {
    pub u: foo,
}
#[repr(C)]
#[derive(Debug, Default, Copy, Clone)]
pub struct foo {
    pub foo: ::std::os::raw::c_int,
}
const _: () = {
    ["Size of foo"][::std::mem::size_of::<foo>() - 4usize];
    ["Alignment of foo"][::std::mem::align_of::<foo>() - 4usize];
    ["Offset of field: foo::foo"][::std::mem::offset_of!(foo, foo) - 0usize];
};
const _: () = {
    ["Size of bar"][::std::mem::size_of::<bar>() - 4usize];
    ["Alignment of bar"][::std::mem::align_of::<bar>() - 4usize];
    ["Offset of field: bar::u"][::std::mem::offset_of!(bar, u) - 0usize];
};
pub type bar_t = bar;
#[repr(C)]
#[derive(Debug, Default, Copy, Clone)]
pub struct baz {
    pub f: foo,
}
const _: () = {
    ["Size of baz"][::std::mem::size_of::<baz>() - 4usize];
    ["Alignment of baz"][::std::mem::align_of::<baz>() - 4usize];
    ["Offset of field: baz::f"][::std::mem::offset_of!(baz, f) - 0usize];
};
