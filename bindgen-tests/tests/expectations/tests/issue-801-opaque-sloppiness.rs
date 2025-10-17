#![allow(dead_code, non_snake_case, non_camel_case_types, non_upper_case_globals)]
#[repr(C)]
#[derive(Debug)]
pub struct A {
    _unused: [u8; 0],
}
#[repr(C)]
#[repr(align(1))]
#[derive(Debug, Default, Copy, Clone, Hash, PartialEq, Eq)]
pub struct B {
    pub _bindgen_opaque_blob: u8,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of B"][::std::mem::size_of::<B>() - 1usize];
    ["Alignment of B"][::std::mem::align_of::<B>() - 1usize];
};
unsafe extern "C" {
    #[link_name = "\u{1}_ZN1B1aE"]
    pub static mut B_a: A;
}
#[repr(C)]
#[derive(Debug, Default, Copy, Clone, Hash, PartialEq, Eq)]
pub struct C {
    pub b: B,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of C"][::std::mem::size_of::<C>() - 1usize];
    ["Alignment of C"][::std::mem::align_of::<C>() - 1usize];
    ["Offset of field: C::b"][::std::mem::offset_of!(C, b) - 0usize];
};
