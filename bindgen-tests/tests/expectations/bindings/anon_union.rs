#![allow(
    dead_code,
    non_snake_case,
    non_camel_case_types,
    non_upper_case_globals
)]

#[repr(C)]
pub struct TErrorResult {
    pub mResult: ::std::os::raw::c_int,
    pub __bindgen_anon_1: TErrorResult__bindgen_ty_1,
    pub mMightHaveUnreported: bool,
    pub mUnionState: TErrorResult_UnionState,
}
impl TErrorResult_UnionState {
    pub const HasException: TErrorResult_UnionState =
        TErrorResult_UnionState::HasMessage;
}
#[repr(i32)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum TErrorResult_UnionState {
    HasMessage = 0,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct TErrorResult_Message {
    _unused: [u8; 0],
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
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
        let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
impl Default for TErrorResult {
    fn default() -> Self {
        let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
#[repr(C)]
pub struct ErrorResult {
    pub _base: TErrorResult,
}
#[test]
fn bindgen_test_layout_ErrorResult() {
    assert_eq!(
        ::std::mem::size_of::<ErrorResult>(),
        24usize,
        concat!("Size of: ", stringify!(ErrorResult))
    );
    assert_eq!(
        ::std::mem::align_of::<ErrorResult>(),
        8usize,
        concat!("Alignment of ", stringify!(ErrorResult))
    );
}
impl Default for ErrorResult {
    fn default() -> Self {
        let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
#[test]
fn __bindgen_test_layout_TErrorResult_open0_int_close0_instantiation() {
    assert_eq!(
        ::std::mem::size_of::<TErrorResult>(),
        24usize,
        concat!(
            "Size of template specialization: ",
            stringify!(TErrorResult)
        )
    );
    assert_eq!(
        ::std::mem::align_of::<TErrorResult>(),
        8usize,
        concat!(
            "Alignment of template specialization: ",
            stringify!(TErrorResult)
        )
    );
}
