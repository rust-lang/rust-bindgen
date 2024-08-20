#![allow(dead_code, non_snake_case, non_camel_case_types, non_upper_case_globals)]
#[repr(C)]
#[derive(Debug, Default, Copy, Clone)]
pub struct A {
    pub _address: u8,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of A"][::std::mem::size_of::<A>() - 1usize];
    ["Alignment of A"][::std::mem::align_of::<A>() - 1usize];
};
extern "C" {
    #[link_name = "\u{1}_ZN1A17inline_definitionEv"]
    pub fn A_inline_definition(this: *mut A);
}
extern "C" {
    #[link_name = "\u{1}_ZN1A22out_of_line_definitionEv"]
    pub fn A_out_of_line_definition(this: *mut A);
}
impl A {
    #[inline]
    pub unsafe fn inline_definition(&mut self) {
        A_inline_definition(self)
    }
    #[inline]
    pub unsafe fn out_of_line_definition(&mut self) {
        A_out_of_line_definition(self)
    }
}
#[repr(C)]
#[derive(Debug, Default, Copy, Clone)]
pub struct B {
    pub _address: u8,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of B"][::std::mem::size_of::<B>() - 1usize];
    ["Alignment of B"][::std::mem::align_of::<B>() - 1usize];
};
#[repr(C)]
#[derive(Debug, Default, Copy, Clone)]
pub struct C {
    pub _address: u8,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of C"][::std::mem::size_of::<C>() - 1usize];
    ["Alignment of C"][::std::mem::align_of::<C>() - 1usize];
};
extern "C" {
    #[link_name = "\u{1}_ZN1CC1ERS_"]
    pub fn C_C(this: *mut C, arg1: *mut C);
}
impl C {
    #[inline]
    pub unsafe fn new(arg1: *mut C) -> Self {
        let mut __bindgen_tmp = ::std::mem::MaybeUninit::uninit();
        C_C(__bindgen_tmp.as_mut_ptr(), arg1);
        __bindgen_tmp.assume_init()
    }
}
