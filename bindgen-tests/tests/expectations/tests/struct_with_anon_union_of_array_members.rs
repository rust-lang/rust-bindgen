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
pub struct bpf_array {
    pub __bindgen_anon_1: bpf_array__bindgen_ty_1,
}
#[repr(C)]
pub union bpf_array__bindgen_ty_1 {
    pub __bindgen_anon_1: ::std::mem::ManuallyDrop<
        bpf_array__bindgen_ty_1__bindgen_ty_1,
    >,
    pub __bindgen_anon_2: ::std::mem::ManuallyDrop<
        bpf_array__bindgen_ty_1__bindgen_ty_2,
    >,
}
#[repr(C)]
#[derive(Debug, Default)]
pub struct bpf_array__bindgen_ty_1__bindgen_ty_1 {
    pub __empty_value1: bpf_array__bindgen_ty_1__bindgen_ty_1__bindgen_ty_1,
    pub value1: __IncompleteArrayField<::std::os::raw::c_char>,
}
#[repr(C)]
#[derive(Debug, Default, Copy, Clone)]
pub struct bpf_array__bindgen_ty_1__bindgen_ty_1__bindgen_ty_1 {}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    [
        "Size of bpf_array__bindgen_ty_1__bindgen_ty_1__bindgen_ty_1",
    ][::std::mem::size_of::<bpf_array__bindgen_ty_1__bindgen_ty_1__bindgen_ty_1>()
        - 0usize];
    [
        "Alignment of bpf_array__bindgen_ty_1__bindgen_ty_1__bindgen_ty_1",
    ][::std::mem::align_of::<bpf_array__bindgen_ty_1__bindgen_ty_1__bindgen_ty_1>()
        - 1usize];
};
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    [
        "Size of bpf_array__bindgen_ty_1__bindgen_ty_1",
    ][::std::mem::size_of::<bpf_array__bindgen_ty_1__bindgen_ty_1>() - 0usize];
    [
        "Alignment of bpf_array__bindgen_ty_1__bindgen_ty_1",
    ][::std::mem::align_of::<bpf_array__bindgen_ty_1__bindgen_ty_1>() - 1usize];
    [
        "Offset of field: bpf_array__bindgen_ty_1__bindgen_ty_1::__empty_value1",
    ][::std::mem::offset_of!(bpf_array__bindgen_ty_1__bindgen_ty_1, __empty_value1)
        - 0usize];
    [
        "Offset of field: bpf_array__bindgen_ty_1__bindgen_ty_1::value1",
    ][::std::mem::offset_of!(bpf_array__bindgen_ty_1__bindgen_ty_1, value1) - 0usize];
};
#[repr(C)]
#[derive(Debug, Default)]
pub struct bpf_array__bindgen_ty_1__bindgen_ty_2 {
    pub __empty_value2: bpf_array__bindgen_ty_1__bindgen_ty_2__bindgen_ty_1,
    pub value2: __IncompleteArrayField<::std::os::raw::c_char>,
}
#[repr(C)]
#[derive(Debug, Default, Copy, Clone)]
pub struct bpf_array__bindgen_ty_1__bindgen_ty_2__bindgen_ty_1 {}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    [
        "Size of bpf_array__bindgen_ty_1__bindgen_ty_2__bindgen_ty_1",
    ][::std::mem::size_of::<bpf_array__bindgen_ty_1__bindgen_ty_2__bindgen_ty_1>()
        - 0usize];
    [
        "Alignment of bpf_array__bindgen_ty_1__bindgen_ty_2__bindgen_ty_1",
    ][::std::mem::align_of::<bpf_array__bindgen_ty_1__bindgen_ty_2__bindgen_ty_1>()
        - 1usize];
};
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    [
        "Size of bpf_array__bindgen_ty_1__bindgen_ty_2",
    ][::std::mem::size_of::<bpf_array__bindgen_ty_1__bindgen_ty_2>() - 0usize];
    [
        "Alignment of bpf_array__bindgen_ty_1__bindgen_ty_2",
    ][::std::mem::align_of::<bpf_array__bindgen_ty_1__bindgen_ty_2>() - 1usize];
    [
        "Offset of field: bpf_array__bindgen_ty_1__bindgen_ty_2::__empty_value2",
    ][::std::mem::offset_of!(bpf_array__bindgen_ty_1__bindgen_ty_2, __empty_value2)
        - 0usize];
    [
        "Offset of field: bpf_array__bindgen_ty_1__bindgen_ty_2::value2",
    ][::std::mem::offset_of!(bpf_array__bindgen_ty_1__bindgen_ty_2, value2) - 0usize];
};
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    [
        "Size of bpf_array__bindgen_ty_1",
    ][::std::mem::size_of::<bpf_array__bindgen_ty_1>() - 0usize];
    [
        "Alignment of bpf_array__bindgen_ty_1",
    ][::std::mem::align_of::<bpf_array__bindgen_ty_1>() - 1usize];
};
impl Default for bpf_array__bindgen_ty_1 {
    fn default() -> Self {
        let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of bpf_array"][::std::mem::size_of::<bpf_array>() - 0usize];
    ["Alignment of bpf_array"][::std::mem::align_of::<bpf_array>() - 1usize];
};
impl Default for bpf_array {
    fn default() -> Self {
        let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
