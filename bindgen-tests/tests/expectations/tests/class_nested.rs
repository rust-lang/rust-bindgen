#![allow(dead_code, non_snake_case, non_camel_case_types, non_upper_case_globals)]
#[repr(C)]
#[derive(Debug, Default, Copy, Clone, Hash, PartialEq, Eq)]
pub struct A {
    pub member_a: ::std::os::raw::c_int,
}
#[repr(C)]
#[derive(Debug, Default, Copy, Clone, Hash, PartialEq, Eq)]
pub struct A_B {
    pub member_b: ::std::os::raw::c_int,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of A_B"][::std::mem::size_of::<A_B>() - 4usize];
    ["Alignment of A_B"][::std::mem::align_of::<A_B>() - 4usize];
    ["Offset of field: A_B::member_b"][::std::mem::offset_of!(A_B, member_b) - 0usize];
};
#[repr(C)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct A_D<T> {
    pub _phantom_0: ::std::marker::PhantomData<::std::cell::UnsafeCell<T>>,
    pub foo: T,
}
impl<T> Default for A_D<T> {
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
    ["Size of A"][::std::mem::size_of::<A>() - 4usize];
    ["Alignment of A"][::std::mem::align_of::<A>() - 4usize];
    ["Offset of field: A::member_a"][::std::mem::offset_of!(A, member_a) - 0usize];
};
#[repr(C)]
#[derive(Debug, Default, Copy, Clone, Hash, PartialEq, Eq)]
pub struct A_C {
    pub baz: ::std::os::raw::c_int,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of A_C"][::std::mem::size_of::<A_C>() - 4usize];
    ["Alignment of A_C"][::std::mem::align_of::<A_C>() - 4usize];
    ["Offset of field: A_C::baz"][::std::mem::offset_of!(A_C, baz) - 0usize];
};
extern "C" {
    pub static mut var: A_B;
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    [
        "Size of template specialization: A_D_open0_int_close0",
    ][::std::mem::size_of::<A_D<::std::os::raw::c_int>>() - 4usize];
    [
        "Align of template specialization: A_D_open0_int_close0",
    ][::std::mem::align_of::<A_D<::std::os::raw::c_int>>() - 4usize];
};
extern "C" {
    pub static mut baz: A_D<::std::os::raw::c_int>;
}
#[repr(C)]
#[derive(Debug, Default, Copy, Clone, Hash, PartialEq, Eq)]
pub struct D {
    pub member: A_B,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of D"][::std::mem::size_of::<D>() - 4usize];
    ["Alignment of D"][::std::mem::align_of::<D>() - 4usize];
    ["Offset of field: D::member"][::std::mem::offset_of!(D, member) - 0usize];
};
#[repr(C)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct Templated<T> {
    pub _phantom_0: ::std::marker::PhantomData<::std::cell::UnsafeCell<T>>,
    pub member: T,
}
#[repr(C)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct Templated_Templated_inner<T> {
    pub _phantom_0: ::std::marker::PhantomData<::std::cell::UnsafeCell<T>>,
    pub member_ptr: *mut T,
}
impl<T> Default for Templated_Templated_inner<T> {
    fn default() -> Self {
        let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
impl<T> Default for Templated<T> {
    fn default() -> Self {
        let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
