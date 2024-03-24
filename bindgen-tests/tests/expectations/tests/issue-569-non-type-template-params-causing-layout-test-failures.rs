#![allow(dead_code, non_snake_case, non_camel_case_types, non_upper_case_globals)]
pub const ENUM_VARIANT_1: _bindgen_ty_1 = _bindgen_ty_1::ENUM_VARIANT_1;
pub const ENUM_VARIANT_2: _bindgen_ty_1 = _bindgen_ty_1::ENUM_VARIANT_2;
#[repr(u32)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum _bindgen_ty_1 {
    ENUM_VARIANT_1 = 0,
    ENUM_VARIANT_2 = 1,
}
pub type JS_Alias = u8;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct JS_Base {
    pub f: JS_Alias,
}
impl Default for JS_Base {
    fn default() -> Self {
        let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct JS_AutoIdVector {
    pub _base: JS_Base,
}
const _: () = {
    ["Size of JS_AutoIdVector"][::std::mem::size_of::<JS_AutoIdVector>() - 1usize];
    ["Alignment of JS_AutoIdVector"][::std::mem::align_of::<JS_AutoIdVector>() - 1usize];
};
impl Default for JS_AutoIdVector {
    fn default() -> Self {
        let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
const _: () = {
    [
        "Size of template specialization: JS_Base_open0_int_close0",
    ][::std::mem::size_of::<JS_Base>() - 1usize];
    [
        "Align of template specialization: JS_Base_open0_int_close0",
    ][::std::mem::align_of::<JS_Base>() - 1usize];
};
