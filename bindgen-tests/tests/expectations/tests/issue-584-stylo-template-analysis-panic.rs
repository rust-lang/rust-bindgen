#![allow(dead_code, non_snake_case, non_camel_case_types, non_upper_case_globals)]
pub type RefPtr<T> = T;
#[repr(C)]
#[derive(Debug, Default, Copy, Clone)]
pub struct A {
    pub _address: u8,
}
pub type A_a = b;
const _: () = {
    assert!(::std::mem::size_of::<A>() == 1usize, "Size of A");
    assert!(::std::mem::align_of::<A>() == 1usize, "Alignment of A");
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
const _: () = {
    assert!(::std::mem::size_of::<g>() == 1usize, "Size of g");
    assert!(::std::mem::align_of::<g>() == 1usize, "Alignment of g");
    assert!(::std::mem::offset_of!(g, h) == 0usize, "Offset of field: g::h");
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
const _: () = {
    assert!(::std::mem::size_of::<b>() == 1usize, "Size of b");
    assert!(::std::mem::align_of::<b>() == 1usize, "Alignment of b");
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
const _: () = {
    assert!(
        ::std::mem::size_of::<f>() == 1usize,
        "Size of template specialization: f_open0_e_open1_int_close1_close0",
    );
    assert!(
        ::std::mem::align_of::<f>() == 1usize,
        "Align of template specialization: f_open0_e_open1_int_close1_close0",
    );
};
