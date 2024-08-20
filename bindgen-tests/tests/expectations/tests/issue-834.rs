#![allow(dead_code, non_snake_case, non_camel_case_types, non_upper_case_globals)]
#[repr(C)]
#[derive(Debug, Default, Copy, Clone)]
pub struct U {
    pub _address: u8,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of U"][::std::mem::size_of::<U>() - 1usize];
    ["Alignment of U"][::std::mem::align_of::<U>() - 1usize];
};
