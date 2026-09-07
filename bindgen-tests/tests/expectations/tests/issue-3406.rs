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
#[repr(align(16))]
#[derive(Debug, Default, Copy, Clone)]
pub struct Inner {
    pub byte: ::std::os::raw::c_char,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of Inner"][::std::mem::size_of::<Inner>() - 16usize];
    ["Alignment of Inner"][::std::mem::align_of::<Inner>() - 16usize];
    ["Offset of field: Inner::byte"][::std::mem::offset_of!(Inner, byte) - 0usize];
};
pub type AlignedInt = ::std::os::raw::c_int;
pub type NestedAlignedInt = AlignedInt;
#[repr(C)]
#[repr(align(16))]
#[derive(Debug, Default, Copy, Clone)]
pub struct Outer {
    pub before: ::std::os::raw::c_int,
    pub inner: Inner,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of Outer"][::std::mem::size_of::<Outer>() - 32usize];
    ["Alignment of Outer"][::std::mem::align_of::<Outer>() - 16usize];
    ["Offset of field: Outer::before"][::std::mem::offset_of!(Outer, before) - 0usize];
    ["Offset of field: Outer::inner"][::std::mem::offset_of!(Outer, inner) - 16usize];
};
#[repr(C)]
#[repr(align(16))]
#[derive(Debug, Default, Copy, Clone)]
pub struct Outer2 {
    pub before: ::std::os::raw::c_int,
    pub __bindgen_padding_0: [u8; 12usize],
    pub one: AlignedInt,
    pub __bindgen_padding_1: [u8; 12usize],
    pub two: AlignedInt,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of Outer2"][::std::mem::size_of::<Outer2>() - 48usize];
    ["Alignment of Outer2"][::std::mem::align_of::<Outer2>() - 16usize];
    ["Offset of field: Outer2::before"][::std::mem::offset_of!(Outer2, before) - 0usize];
    ["Offset of field: Outer2::one"][::std::mem::offset_of!(Outer2, one) - 16usize];
    ["Offset of field: Outer2::two"][::std::mem::offset_of!(Outer2, two) - 32usize];
};
#[repr(C)]
#[repr(align(16))]
#[derive(Debug, Default, Copy, Clone)]
pub struct Outer3 {
    pub before: ::std::os::raw::c_int,
    pub __bindgen_padding_0: [u8; 12usize],
    pub inner: AlignedInt,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of Outer3"][::std::mem::size_of::<Outer3>() - 32usize];
    ["Alignment of Outer3"][::std::mem::align_of::<Outer3>() - 16usize];
    ["Offset of field: Outer3::before"][::std::mem::offset_of!(Outer3, before) - 0usize];
    ["Offset of field: Outer3::inner"][::std::mem::offset_of!(Outer3, inner) - 16usize];
};
#[repr(C)]
#[repr(align(16))]
#[derive(Debug, Default, Copy, Clone)]
pub struct Outer4 {
    pub before: ::std::os::raw::c_int,
    pub __bindgen_padding_0: [u8; 12usize],
    pub inner: NestedAlignedInt,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of Outer4"][::std::mem::size_of::<Outer4>() - 32usize];
    ["Alignment of Outer4"][::std::mem::align_of::<Outer4>() - 16usize];
    ["Offset of field: Outer4::before"][::std::mem::offset_of!(Outer4, before) - 0usize];
    ["Offset of field: Outer4::inner"][::std::mem::offset_of!(Outer4, inner) - 16usize];
};
#[repr(C)]
#[derive(Debug, Default, Copy, Clone)]
pub struct Inner2 {
    pub a: ::std::os::raw::c_longlong,
    pub b: ::std::os::raw::c_longlong,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of Inner2"][::std::mem::size_of::<Inner2>() - 16usize];
    ["Alignment of Inner2"][::std::mem::align_of::<Inner2>() - 8usize];
    ["Offset of field: Inner2::a"][::std::mem::offset_of!(Inner2, a) - 0usize];
    ["Offset of field: Inner2::b"][::std::mem::offset_of!(Inner2, b) - 8usize];
};
pub type AlignedInner2 = Inner2;
#[repr(C)]
#[repr(align(16))]
#[derive(Debug, Default, Copy, Clone)]
pub struct Outer5 {
    pub before: ::std::os::raw::c_longlong,
    pub __bindgen_padding_0: [u8; 8usize],
    pub inner: [AlignedInner2; 1usize],
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of Outer5"][::std::mem::size_of::<Outer5>() - 32usize];
    ["Alignment of Outer5"][::std::mem::align_of::<Outer5>() - 16usize];
    ["Offset of field: Outer5::before"][::std::mem::offset_of!(Outer5, before) - 0usize];
    ["Offset of field: Outer5::inner"][::std::mem::offset_of!(Outer5, inner) - 16usize];
};
#[repr(C)]
#[repr(align(16))]
#[derive(Debug, Default)]
pub struct Outer6 {
    pub before: ::std::os::raw::c_longlong,
    pub __bindgen_padding_0: [u8; 8usize],
    pub inner: __IncompleteArrayField<AlignedInner2>,
    pub tail: ::std::os::raw::c_char,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of Outer6"][::std::mem::size_of::<Outer6>() - 32usize];
    ["Alignment of Outer6"][::std::mem::align_of::<Outer6>() - 16usize];
    ["Offset of field: Outer6::before"][::std::mem::offset_of!(Outer6, before) - 0usize];
    ["Offset of field: Outer6::inner"][::std::mem::offset_of!(Outer6, inner) - 16usize];
    ["Offset of field: Outer6::tail"][::std::mem::offset_of!(Outer6, tail) - 16usize];
};
pub const AlignedEnum_Value: AlignedEnum = 1;
pub type AlignedEnum = ::std::os::raw::c_uint;
#[repr(C)]
#[repr(align(16))]
#[derive(Debug, Copy, Clone)]
pub struct Outer7 {
    pub before: ::std::os::raw::c_longlong,
    pub __bindgen_padding_0: [u8; 8usize],
    pub inner: AlignedEnum,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of Outer7"][::std::mem::size_of::<Outer7>() - 32usize];
    ["Alignment of Outer7"][::std::mem::align_of::<Outer7>() - 16usize];
    ["Offset of field: Outer7::before"][::std::mem::offset_of!(Outer7, before) - 0usize];
    ["Offset of field: Outer7::inner"][::std::mem::offset_of!(Outer7, inner) - 16usize];
};
impl Default for Outer7 {
    fn default() -> Self {
        let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
