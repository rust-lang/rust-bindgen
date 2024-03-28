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
    pub u: u8,
}
const _: () = {
    assert!(::std::mem::size_of::<A>() == 1usize, "Size of A");
    assert!(::std::mem::align_of::<A>() == 1usize, "Alignment of A");
    assert!(::std::mem::offset_of!(A, u) == 0usize, "Offset of field: A::u");
};
const _: () = {
    assert!(
        ::std::mem::size_of::<C>() == 1usize,
        "Size of template specialization: C_open0_A_close0",
    );
    assert!(
        ::std::mem::align_of::<C>() == 1usize,
        "Align of template specialization: C_open0_A_close0",
    );
};
