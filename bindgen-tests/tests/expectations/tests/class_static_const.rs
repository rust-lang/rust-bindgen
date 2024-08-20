#![allow(dead_code, non_snake_case, non_camel_case_types, non_upper_case_globals)]
#[repr(C)]
#[derive(Debug, Default, Copy, Clone, Hash, PartialEq, Eq)]
pub struct A {
    pub _address: u8,
}
pub const A_a: ::std::os::raw::c_int = 0;
pub const A_b: i32 = 63;
pub const A_c: u32 = 255;
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of A"][::std::mem::size_of::<A>() - 1usize];
    ["Alignment of A"][::std::mem::align_of::<A>() - 1usize];
};
