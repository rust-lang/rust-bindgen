#![allow(dead_code, non_snake_case, non_camel_case_types, non_upper_case_globals)]
#[repr(C)]
#[derive(Clone, Copy, Debug, Default)]
pub struct A {
    pub _address: u8,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of A"][::std::mem::size_of::<A>() - 1usize];
    ["Alignment of A"][::std::mem::align_of::<A>() - 1usize];
};
#[repr(C)]
#[derive(Clone, Copy, Debug, Default)]
pub struct A_I {
    pub i: ::std::os::raw::c_int,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of A_I"][::std::mem::size_of::<A_I>() - 4usize];
    ["Alignment of A_I"][::std::mem::align_of::<A_I>() - 4usize];
    ["Offset of field: A_I::i"][::std::mem::offset_of!(A_I, i) - 0usize];
};
