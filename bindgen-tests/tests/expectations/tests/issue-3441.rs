#![allow(dead_code, non_snake_case, non_camel_case_types, non_upper_case_globals)]
#[repr(C)]
#[derive(Debug, Default, Copy, Clone)]
pub struct S {
    pub _address: u8,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of S"][::std::mem::size_of::<S>() - 1usize];
    ["Alignment of S"][::std::mem::align_of::<S>() - 1usize];
};
unsafe extern "C" {
    #[link_name = "\u{1}_ZNH1S5valueES_i"]
    pub fn S_value(self_: S, y: ::std::os::raw::c_int) -> ::std::os::raw::c_int;
}
unsafe extern "C" {
    #[link_name = "\u{1}_ZNH1S9referenceERS_i"]
    pub fn S_reference(self_: *mut S, y: ::std::os::raw::c_int) -> ::std::os::raw::c_int;
}
unsafe extern "C" {
    #[link_name = "\u{1}_ZN1S7regularEi"]
    pub fn S_regular(this: *mut S, y: ::std::os::raw::c_int) -> ::std::os::raw::c_int;
}
impl S {
    #[inline]
    pub unsafe fn value(self_: S, y: ::std::os::raw::c_int) -> ::std::os::raw::c_int {
        S_value(self_, y)
    }
    #[inline]
    pub unsafe fn reference(
        self_: *mut S,
        y: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int {
        S_reference(self_, y)
    }
    #[inline]
    pub unsafe fn regular(&mut self, y: ::std::os::raw::c_int) -> ::std::os::raw::c_int {
        S_regular(self, y)
    }
}
