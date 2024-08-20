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
#[repr(C)]
#[derive(Debug, Default)]
pub struct dm_deps {
    pub count: ::std::os::raw::c_uint,
    pub filler: ::std::os::raw::c_uint,
    pub device: __IncompleteArrayField<::std::os::raw::c_ulonglong>,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of dm_deps"][::std::mem::size_of::<dm_deps>() - 8usize];
    ["Alignment of dm_deps"][::std::mem::align_of::<dm_deps>() - 8usize];
    ["Offset of field: dm_deps::count"][::std::mem::offset_of!(dm_deps, count) - 0usize];
    [
        "Offset of field: dm_deps::filler",
    ][::std::mem::offset_of!(dm_deps, filler) - 4usize];
    [
        "Offset of field: dm_deps::device",
    ][::std::mem::offset_of!(dm_deps, device) - 8usize];
};
