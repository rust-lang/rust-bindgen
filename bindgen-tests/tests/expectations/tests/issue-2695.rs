#![allow(dead_code, non_snake_case, non_camel_case_types, non_upper_case_globals)]
#[repr(C, packed(2))]
#[derive(Debug, Default, Copy, Clone)]
pub struct Test {
    pub x: ::std::os::raw::c_ulong,
    pub a: ::std::os::raw::c_char,
    pub b: ::std::os::raw::c_char,
    pub c: ::std::os::raw::c_char,
    pub __bindgen_padding_0: u8,
}
const _: () = {
    ["Size of Test"][::std::mem::size_of::<Test>() - 12usize];
    ["Alignment of Test"][::std::mem::align_of::<Test>() - 2usize];
    ["Offset of field: Test::x"][::std::mem::offset_of!(Test, x) - 0usize];
    ["Offset of field: Test::a"][::std::mem::offset_of!(Test, a) - 8usize];
    ["Offset of field: Test::b"][::std::mem::offset_of!(Test, b) - 9usize];
    ["Offset of field: Test::c"][::std::mem::offset_of!(Test, c) - 10usize];
};
