#![allow(dead_code, non_snake_case, non_camel_case_types, non_upper_case_globals)]
/// <div rustbindgen replaces="nsTArray"></div>
#[repr(C)]
#[derive(Debug, Default, Copy, Clone)]
pub struct nsTArray {
    pub y: ::std::os::raw::c_uint,
}
#[repr(C)]
#[derive(Debug, Default, Copy, Clone)]
pub struct Test {
    pub a: nsTArray,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of Test"][::std::mem::size_of::<Test>() - 4usize];
    ["Alignment of Test"][::std::mem::align_of::<Test>() - 4usize];
    ["Offset of field: Test::a"][::std::mem::offset_of!(Test, a) - 0usize];
};
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    [
        "Size of template specialization: nsTArray_open0_long_close0",
    ][::std::mem::size_of::<nsTArray>() - 4usize];
    [
        "Align of template specialization: nsTArray_open0_long_close0",
    ][::std::mem::align_of::<nsTArray>() - 4usize];
};
