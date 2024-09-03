#![allow(dead_code, non_snake_case, non_camel_case_types, non_upper_case_globals)]
pub type RefPtr<T> = T;
#[repr(C)]
#[derive(Debug, Default, Copy, Clone)]
pub struct A {
    pub _address: u8,
}
pub type A_a = b;
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of A"][::std::mem::size_of::<A>() - 1usize];
    ["Alignment of A"][::std::mem::align_of::<A>() - 1usize];
};
#[repr(C)]
pub struct e<c> {
    pub _phantom_0: ::std::marker::PhantomData<::std::cell::UnsafeCell<c>>,
    pub d: RefPtr<c>,
}
impl<c> Default for e<c> {
    fn default() -> Self {
        let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
#[repr(C)]
#[derive(Debug, Default, Copy, Clone)]
pub struct f {
    pub _address: u8,
}
#[repr(C)]
pub struct g {
    pub h: f,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of g"][::std::mem::size_of::<g>() - 1usize];
    ["Alignment of g"][::std::mem::align_of::<g>() - 1usize];
    ["Offset of field: g::h"][::std::mem::offset_of!(g, h) - 0usize];
};
impl Default for g {
    fn default() -> Self {
        let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
#[repr(C)]
pub struct b {
    pub _base: g,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of b"][::std::mem::size_of::<b>() - 1usize];
    ["Alignment of b"][::std::mem::align_of::<b>() - 1usize];
};
impl Default for b {
    fn default() -> Self {
        let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
extern "C" {
    #[link_name = "\u{1}_Z25Servo_Element_GetSnapshotv"]
    pub fn Servo_Element_GetSnapshot() -> A;
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    [
        "Size of template specialization: f_open0_e_open1_int_close1_close0",
    ][::std::mem::size_of::<f>() - 1usize];
    [
        "Align of template specialization: f_open0_e_open1_int_close1_close0",
    ][::std::mem::align_of::<f>() - 1usize];
};
