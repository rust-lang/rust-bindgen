#![allow(dead_code, non_snake_case, non_camel_case_types, non_upper_case_globals)]
/// Template definition that doesn't contain float can derive Hash/PartialOrd/Ord/PartialEq/Eq
#[repr(C)]
#[derive(Debug, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
pub struct foo<T> {
    pub _phantom_0: ::std::marker::PhantomData<::std::cell::UnsafeCell<T>>,
    pub data: T,
}
impl<T> Default for foo<T> {
    fn default() -> Self {
        let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
/// Can derive Hash/PartialOrd/Ord/PartialEq/Eq when instantiated with int
#[repr(C)]
#[derive(Debug, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
pub struct IntStr {
    pub a: foo<::std::os::raw::c_int>,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of IntStr"][::std::mem::size_of::<IntStr>() - 4usize];
    ["Alignment of IntStr"][::std::mem::align_of::<IntStr>() - 4usize];
    ["Offset of field: IntStr::a"][::std::mem::offset_of!(IntStr, a) - 0usize];
};
impl Default for IntStr {
    fn default() -> Self {
        let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
/// Cannot derive Hash/Eq/Ord when instantiated with float but can derive PartialEq/PartialOrd
#[repr(C)]
#[derive(Debug, Copy, Clone, PartialOrd, PartialEq)]
pub struct FloatStr {
    pub a: foo<f32>,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of FloatStr"][::std::mem::size_of::<FloatStr>() - 4usize];
    ["Alignment of FloatStr"][::std::mem::align_of::<FloatStr>() - 4usize];
    ["Offset of field: FloatStr::a"][::std::mem::offset_of!(FloatStr, a) - 0usize];
};
impl Default for FloatStr {
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
    [
        "Size of template specialization: foo_open0_int_close0",
    ][::std::mem::size_of::<foo<::std::os::raw::c_int>>() - 4usize];
    [
        "Align of template specialization: foo_open0_int_close0",
    ][::std::mem::align_of::<foo<::std::os::raw::c_int>>() - 4usize];
};
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    [
        "Size of template specialization: foo_open0_float_close0",
    ][::std::mem::size_of::<foo<f32>>() - 4usize];
    [
        "Align of template specialization: foo_open0_float_close0",
    ][::std::mem::align_of::<foo<f32>>() - 4usize];
};
