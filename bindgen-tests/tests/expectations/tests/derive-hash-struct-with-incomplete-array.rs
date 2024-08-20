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
pub struct test {
    pub a: ::std::os::raw::c_int,
    pub zero_length_array: __IncompleteArrayField<::std::os::raw::c_char>,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of test"][::std::mem::size_of::<test>() - 4usize];
    ["Alignment of test"][::std::mem::align_of::<test>() - 4usize];
    ["Offset of field: test::a"][::std::mem::offset_of!(test, a) - 0usize];
    [
        "Offset of field: test::zero_length_array",
    ][::std::mem::offset_of!(test, zero_length_array) - 4usize];
};
#[repr(C)]
#[derive(Debug, Default)]
pub struct test2 {
    pub a: ::std::os::raw::c_int,
    pub incomplete_array: __IncompleteArrayField<::std::os::raw::c_char>,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of test2"][::std::mem::size_of::<test2>() - 4usize];
    ["Alignment of test2"][::std::mem::align_of::<test2>() - 4usize];
    ["Offset of field: test2::a"][::std::mem::offset_of!(test2, a) - 0usize];
    [
        "Offset of field: test2::incomplete_array",
    ][::std::mem::offset_of!(test2, incomplete_array) - 4usize];
};
#[repr(C)]
#[derive(Debug, Default)]
pub struct test3 {
    pub a: ::std::os::raw::c_int,
    pub zero_length_array: __IncompleteArrayField<::std::os::raw::c_char>,
    pub incomplete_array: __IncompleteArrayField<::std::os::raw::c_char>,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of test3"][::std::mem::size_of::<test3>() - 4usize];
    ["Alignment of test3"][::std::mem::align_of::<test3>() - 4usize];
    ["Offset of field: test3::a"][::std::mem::offset_of!(test3, a) - 0usize];
    [
        "Offset of field: test3::zero_length_array",
    ][::std::mem::offset_of!(test3, zero_length_array) - 4usize];
    [
        "Offset of field: test3::incomplete_array",
    ][::std::mem::offset_of!(test3, incomplete_array) - 4usize];
};
