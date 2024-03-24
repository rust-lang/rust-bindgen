#![allow(dead_code, non_snake_case, non_camel_case_types, non_upper_case_globals)]
pub const k: bool = true;
#[repr(C)]
#[derive(Debug, Default, Copy, Clone)]
pub struct A {
    pub _address: u8,
}
pub const A_k: bool = false;
const _: () = {
    assert!(::std::mem::size_of::<A>() == 1usize, "Size of A");
    assert!(::std::mem::align_of::<A>() == 1usize, "Alignment of A");
};
pub type foo = bool;
pub const k2: foo = true;
