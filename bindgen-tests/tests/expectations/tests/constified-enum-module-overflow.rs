#![allow(dead_code, non_snake_case, non_camel_case_types, non_upper_case_globals)]
#[repr(C)]
#[derive(Debug, Default, Copy, Clone)]
pub struct B {
    pub _address: u8,
}
#[repr(C)]
#[derive(Debug, Default, Copy, Clone)]
pub struct C {
    pub _address: u8,
}
pub type C_U = B;
#[repr(C)]
#[derive(Debug, Default, Copy, Clone)]
pub struct A {
    pub u: B,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of A"][::std::mem::size_of::<A>() - 1usize];
    ["Alignment of A"][::std::mem::align_of::<A>() - 1usize];
    ["Offset of field: A::u"][::std::mem::offset_of!(A, u) - 0usize];
};
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    [
        "Size of template specialization: C_open0_A_close0",
    ][::std::mem::size_of::<C>() - 1usize];
    [
        "Align of template specialization: C_open0_A_close0",
    ][::std::mem::align_of::<C>() - 1usize];
};
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    [
        "Size of template specialization: B_open0_A_close0",
    ][::std::mem::size_of::<B>() - 1usize];
    [
        "Align of template specialization: B_open0_A_close0",
    ][::std::mem::align_of::<B>() - 1usize];
};
