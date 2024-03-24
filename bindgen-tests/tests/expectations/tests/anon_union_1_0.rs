#![allow(dead_code, non_snake_case, non_camel_case_types, non_upper_case_globals)]
#[repr(C)]
pub struct __BindgenUnionField<T>(::std::marker::PhantomData<T>);
impl<T> __BindgenUnionField<T> {
    #[inline]
    pub fn new() -> Self {
        __BindgenUnionField(::std::marker::PhantomData)
    }
    #[inline]
    pub unsafe fn as_ref(&self) -> &T {
        ::std::mem::transmute(self)
    }
    #[inline]
    pub unsafe fn as_mut(&mut self) -> &mut T {
        ::std::mem::transmute(self)
    }
}
impl<T> ::std::default::Default for __BindgenUnionField<T> {
    #[inline]
    fn default() -> Self {
        Self::new()
    }
}
impl<T> ::std::clone::Clone for __BindgenUnionField<T> {
    #[inline]
    fn clone(&self) -> Self {
        *self
    }
}
impl<T> ::std::marker::Copy for __BindgenUnionField<T> {}
impl<T> ::std::fmt::Debug for __BindgenUnionField<T> {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.write_str("__BindgenUnionField")
    }
}
impl<T> ::std::hash::Hash for __BindgenUnionField<T> {
    fn hash<H: ::std::hash::Hasher>(&self, _state: &mut H) {}
}
impl<T> ::std::cmp::PartialEq for __BindgenUnionField<T> {
    fn eq(&self, _other: &__BindgenUnionField<T>) -> bool {
        true
    }
}
impl<T> ::std::cmp::Eq for __BindgenUnionField<T> {}
#[repr(C)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct TErrorResult {
    pub mResult: ::std::os::raw::c_int,
    pub __bindgen_anon_1: TErrorResult__bindgen_ty_1,
    pub mMightHaveUnreported: bool,
    pub mUnionState: TErrorResult_UnionState,
}
pub const TErrorResult_UnionState_HasException: TErrorResult_UnionState = TErrorResult_UnionState::HasMessage;
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
#[derive(Debug, Default, Copy, Clone, Hash, PartialEq, Eq)]
pub struct TErrorResult__bindgen_ty_1 {
    pub mMessage: __BindgenUnionField<*mut TErrorResult_Message>,
    pub mDOMExceptionInfo: __BindgenUnionField<*mut TErrorResult_DOMExceptionInfo>,
    pub bindgen_union_field: u64,
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
#[derive(Debug, Copy, Hash, PartialEq, Eq)]
pub struct ErrorResult {
    pub _base: TErrorResult,
}
#[test]
fn bindgen_test_layout_ErrorResult() {
    assert_eq!(::std::mem::size_of::<ErrorResult>(), 24usize, "Size of ErrorResult");
    assert_eq!(
        ::std::mem::align_of::<ErrorResult>(),
        8usize,
        "Alignment of ErrorResult",
    );
}
impl Clone for ErrorResult {
    fn clone(&self) -> Self {
        *self
    }
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
#[test]
fn __bindgen_test_layout_TErrorResult_open0_int_close0_instantiation() {
    assert_eq!(
        ::std::mem::size_of::<TErrorResult>(),
        24usize,
        "Size of template specialization: TErrorResult_open0_int_close0",
    );
    assert_eq!(
        ::std::mem::align_of::<TErrorResult>(),
        8usize,
        "Align of template specialization: TErrorResult_open0_int_close0",
    );
}
