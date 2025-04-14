#![allow(dead_code, non_snake_case, non_camel_case_types, non_upper_case_globals)]
#[repr(C)]
#[derive(Debug, Default, Copy, Clone)]
pub struct A {
    pub _address: u8,
}
pub const A_B_B1: A_B = 0;
pub const A_B_B2: A_B = 1;
pub type A_B = ::std::os::raw::c_uint;
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of A"][::std::mem::size_of::<A>() - 1usize];
    ["Alignment of A"][::std::mem::align_of::<A>() - 1usize];
};
