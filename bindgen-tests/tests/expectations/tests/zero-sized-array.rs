#![allow(dead_code, non_snake_case, non_camel_case_types, non_upper_case_globals)]
#[repr(C)]
#[derive(Default)]
pub struct __IncompleteArrayField<T>(::std::marker::PhantomData<T>, [T; 0]);
impl<T> __IncompleteArrayField<T> {
    #[inline]
    pub const fn new() -> Self {
        __IncompleteArrayField(::std::marker::PhantomData, [])
    }
    #[inline]
    pub fn as_ptr(&self) -> *const T {
        self as *const _ as *const T
    }
    #[inline]
    pub fn as_mut_ptr(&mut self) -> *mut T {
        self as *mut _ as *mut T
    }
    #[inline]
    pub unsafe fn as_slice(&self, len: usize) -> &[T] {
        ::std::slice::from_raw_parts(self.as_ptr(), len)
    }
    #[inline]
    pub unsafe fn as_mut_slice(&mut self, len: usize) -> &mut [T] {
        ::std::slice::from_raw_parts_mut(self.as_mut_ptr(), len)
    }
}
impl<T> ::std::fmt::Debug for __IncompleteArrayField<T> {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.write_str("__IncompleteArrayField")
    }
}
/// Bizarrely enough, this should *not* get an `_address` field.
#[repr(C)]
#[derive(Debug, Default)]
pub struct ZeroSizedArray {
    pub arr: __IncompleteArrayField<::std::os::raw::c_char>,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of ZeroSizedArray"][::std::mem::size_of::<ZeroSizedArray>() - 0usize];
    ["Alignment of ZeroSizedArray"][::std::mem::align_of::<ZeroSizedArray>() - 1usize];
    [
        "Offset of field: ZeroSizedArray::arr",
    ][::std::mem::offset_of!(ZeroSizedArray, arr) - 0usize];
};
/// And nor should this get an `_address` field.
#[repr(C)]
#[derive(Debug, Default)]
pub struct ContainsZeroSizedArray {
    pub zsa: ZeroSizedArray,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    [
        "Size of ContainsZeroSizedArray",
    ][::std::mem::size_of::<ContainsZeroSizedArray>() - 0usize];
    [
        "Alignment of ContainsZeroSizedArray",
    ][::std::mem::align_of::<ContainsZeroSizedArray>() - 1usize];
    [
        "Offset of field: ContainsZeroSizedArray::zsa",
    ][::std::mem::offset_of!(ContainsZeroSizedArray, zsa) - 0usize];
};
/// Inheriting from ZeroSizedArray shouldn't cause an `_address` to be inserted
/// either.
#[repr(C)]
#[derive(Debug, Default)]
pub struct InheritsZeroSizedArray {
    pub _base: ZeroSizedArray,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    [
        "Size of InheritsZeroSizedArray",
    ][::std::mem::size_of::<InheritsZeroSizedArray>() - 0usize];
    [
        "Alignment of InheritsZeroSizedArray",
    ][::std::mem::align_of::<InheritsZeroSizedArray>() - 1usize];
};
/// And this should not get an `_address` field either.
#[repr(C)]
#[derive(Debug, Default)]
pub struct DynamicallySizedArray {
    pub arr: __IncompleteArrayField<::std::os::raw::c_char>,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    [
        "Size of DynamicallySizedArray",
    ][::std::mem::size_of::<DynamicallySizedArray>() - 0usize];
    [
        "Alignment of DynamicallySizedArray",
    ][::std::mem::align_of::<DynamicallySizedArray>() - 1usize];
    [
        "Offset of field: DynamicallySizedArray::arr",
    ][::std::mem::offset_of!(DynamicallySizedArray, arr) - 0usize];
};
/// No `_address` field here either.
#[repr(C)]
#[derive(Debug, Default)]
pub struct ContainsDynamicallySizedArray {
    pub dsa: DynamicallySizedArray,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    [
        "Size of ContainsDynamicallySizedArray",
    ][::std::mem::size_of::<ContainsDynamicallySizedArray>() - 0usize];
    [
        "Alignment of ContainsDynamicallySizedArray",
    ][::std::mem::align_of::<ContainsDynamicallySizedArray>() - 1usize];
    [
        "Offset of field: ContainsDynamicallySizedArray::dsa",
    ][::std::mem::offset_of!(ContainsDynamicallySizedArray, dsa) - 0usize];
};
