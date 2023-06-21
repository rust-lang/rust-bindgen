#![allow(dead_code, non_snake_case, non_camel_case_types, non_upper_case_globals)]
#[repr(C)]
#[derive(Debug, Default, Copy, Clone)]
pub struct foo {}
const _: () = {
    ["Size of foo"][::std::mem::size_of::<foo>() - 0usize];
    ["Alignment of foo"][::std::mem::align_of::<foo>() - 1usize];
};
#[repr(C)]
#[derive(Debug, Default, Copy, Clone)]
pub struct bar {
    pub field: foo,
}
const _: () = {
    ["Size of bar"][::std::mem::size_of::<bar>() - 0usize];
    ["Alignment of bar"][::std::mem::align_of::<bar>() - 1usize];
    ["Offset of field: bar::field"][::std::mem::offset_of!(bar, field) - 0usize];
};
#[repr(C)]
#[derive(Debug, Default, Copy, Clone)]
pub struct baz {
    pub field: bar,
}
const _: () = {
    ["Size of baz"][::std::mem::size_of::<baz>() - 0usize];
    ["Alignment of baz"][::std::mem::align_of::<baz>() - 1usize];
    ["Offset of field: baz::field"][::std::mem::offset_of!(baz, field) - 0usize];
};
