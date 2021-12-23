#![allow(
    dead_code,
    non_snake_case,
    non_camel_case_types,
    non_upper_case_globals
)]

pub type RefPtr<T> = T;

#[repr(C)]
#[derive(Debug, Default, Copy, Clone)]
pub struct A {
    pub _address: u8,
}
pub type A_a = b;
#[test]
fn bindgen_test_layout_A() {
    assert_eq!(
        ::std::mem::size_of::<A>(),
        1usize,
        concat!("Size of: ", stringify!(A))
    );
    assert_eq!(
        ::std::mem::align_of::<A>(),
        1usize,
        concat!("Alignment of ", stringify!(A))
    );
}
#[repr(C)]
pub struct e<c> {
    pub d: RefPtr<c>,
    pub _phantom_0: ::std::marker::PhantomData<::std::cell::UnsafeCell<c>>,
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
#[test]
fn bindgen_test_layout_g() {
    assert_eq!(
        ::std::mem::size_of::<g>(),
        1usize,
        concat!("Size of: ", stringify!(g))
    );
    assert_eq!(
        ::std::mem::align_of::<g>(),
        1usize,
        concat!("Alignment of ", stringify!(g))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<g>())).h as *const _ as usize },
        0usize,
        concat!("Offset of field: ", stringify!(g), "::", stringify!(h))
    );
}
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
#[test]
fn bindgen_test_layout_b() {
    assert_eq!(
        ::std::mem::size_of::<b>(),
        1usize,
        concat!("Size of: ", stringify!(b))
    );
    assert_eq!(
        ::std::mem::align_of::<b>(),
        1usize,
        concat!("Alignment of ", stringify!(b))
    );
}
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
#[test]
fn __bindgen_test_layout_f_open0_e_open1_int_close1_close0_instantiation() {
    assert_eq!(
        ::std::mem::size_of::<f>(),
        1usize,
        concat!("Size of template specialization: ", stringify!(f))
    );
    assert_eq!(
        ::std::mem::align_of::<f>(),
        1usize,
        concat!("Alignment of template specialization: ", stringify!(f))
    );
}
