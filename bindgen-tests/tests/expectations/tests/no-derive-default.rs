#![allow(dead_code, non_snake_case, non_camel_case_types, non_upper_case_globals)]
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct foo {
    bar: ::std::os::raw::c_int,
}
/// bar should compile. It will normally derive default, but our blocklist of foo
/// and replacement for another type that doesn't implement it would prevent it
/// from building if --no-derive-default didn't work.
#[repr(C)]
pub struct bar {
    pub foo: foo,
    pub baz: ::std::os::raw::c_int,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of bar"][::std::mem::size_of::<bar>() - 8usize];
    ["Alignment of bar"][::std::mem::align_of::<bar>() - 4usize];
    ["Offset of field: bar::foo"][::std::mem::offset_of!(bar, foo) - 0usize];
    ["Offset of field: bar::baz"][::std::mem::offset_of!(bar, baz) - 4usize];
};
