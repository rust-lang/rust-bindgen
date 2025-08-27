#![allow(dead_code, non_snake_case, non_camel_case_types, non_upper_case_globals)]
#[repr(C)]
#[repr(align(16))]
#[derive(Debug, Default, Copy, Clone)]
pub struct foo {
    pub my_signed: i128,
    pub my_unsigned: u128,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of foo"][::std::mem::size_of::<foo>() - 32usize];
    ["Alignment of foo"][::std::mem::align_of::<foo>() - 16usize];
    ["Offset of field: foo::my_signed"][::std::mem::offset_of!(foo, my_signed) - 0usize];
    [
        "Offset of field: foo::my_unsigned",
    ][::std::mem::offset_of!(foo, my_unsigned) - 16usize];
};
