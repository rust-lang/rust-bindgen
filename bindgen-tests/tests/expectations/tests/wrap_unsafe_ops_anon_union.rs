#![allow(dead_code, non_snake_case, non_camel_case_types, non_upper_case_globals)]
#[repr(C)]
pub struct TErrorResult {
    pub mResult: ::std::os::raw::c_int,
    pub __bindgen_anon_1: TErrorResult__bindgen_ty_1,
    pub mMightHaveUnreported: bool,
    pub mUnionState: TErrorResult_UnionState,
}
pub const TErrorResult_UnionState_HasMessage: TErrorResult_UnionState = 0;
pub const TErrorResult_UnionState_HasException: TErrorResult_UnionState = 0;
pub type TErrorResult_UnionState = i32;
#[repr(C)]
#[derive(Debug)]
pub struct TErrorResult_Message {
    _unused: [u8; 0],
}
#[repr(C)]
#[derive(Debug)]
pub struct TErrorResult_DOMExceptionInfo {
    _unused: [u8; 0],
}
#[repr(C)]
pub union TErrorResult__bindgen_ty_1 {
    pub mMessage: *mut TErrorResult_Message,
    pub mDOMExceptionInfo: *mut TErrorResult_DOMExceptionInfo,
}
impl Default for TErrorResult__bindgen_ty_1 {
    fn default() -> Self {
        unsafe {
            let mut s: Self = ::std::mem::uninitialized();
            ::std::ptr::write_bytes(&mut s, 0, 1);
            s
        }
    }
}
impl Default for TErrorResult {
    fn default() -> Self {
        unsafe {
            let mut s: Self = ::std::mem::uninitialized();
            ::std::ptr::write_bytes(&mut s, 0, 1);
            s
        }
    }
}
#[repr(C)]
pub struct ErrorResult {
    pub _base: TErrorResult,
}
impl Default for ErrorResult {
    fn default() -> Self {
        unsafe {
            let mut s: Self = ::std::mem::uninitialized();
            ::std::ptr::write_bytes(&mut s, 0, 1);
            s
        }
    }
}
