#![allow(
    dead_code,
    non_snake_case,
    non_camel_case_types,
    non_upper_case_globals
)]

#[repr(C)]
#[derive(Debug, Default, Copy, Clone)]
pub struct ClassA {
    pub _address: u8,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct ClassA_ClassAInner<T> {
    pub x: *mut T,
    pub _phantom_0: ::std::marker::PhantomData<::std::cell::UnsafeCell<T>>,
}
impl<T> Default for ClassA_ClassAInner<T> {
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
pub struct ClassB {
    pub _address: u8,
}
#[repr(C)]
#[derive(Debug, Default, Copy, Clone)]
pub struct ClassC {
    pub _address: u8,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct ClassC_ClassCInnerB {
    pub cache: *mut ClassC_ClassCInnerA,
}
impl Default for ClassC_ClassCInnerB {
    fn default() -> Self {
        let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct ClassC_ClassCInnerA {
    pub member: *mut ClassC_ClassCInnerB,
}
impl Default for ClassC_ClassCInnerA {
    fn default() -> Self {
        let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct ClassC_ClassCInnerCRTP {
    pub _address: u8,
}
impl Default for ClassC_ClassCInnerCRTP {
    fn default() -> Self {
        let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct ClassD {
    pub _address: u8,
}
#[test]
fn bindgen_test_layout_ClassD() {
    assert_eq!(
        ::std::mem::size_of::<ClassD>(),
        1usize,
        concat!("Size of: ", stringify!(ClassD))
    );
    assert_eq!(
        ::std::mem::align_of::<ClassD>(),
        1usize,
        concat!("Alignment of ", stringify!(ClassD))
    );
}
impl Default for ClassD {
    fn default() -> Self {
        let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
#[test]
fn __bindgen_test_layout_ClassB_open0_ClassD_ClassCInnerCRTP_close0_instantiation(
) {
    assert_eq!(
        ::std::mem::size_of::<ClassB>(),
        1usize,
        concat!("Size of template specialization: ", stringify!(ClassB))
    );
    assert_eq!(
        ::std::mem::align_of::<ClassB>(),
        1usize,
        concat!("Alignment of template specialization: ", stringify!(ClassB))
    );
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct ClassCInnerCRTP {
    pub _address: u8,
}
#[test]
fn bindgen_test_layout_ClassCInnerCRTP() {
    assert_eq!(
        ::std::mem::size_of::<ClassCInnerCRTP>(),
        1usize,
        concat!("Size of: ", stringify!(ClassCInnerCRTP))
    );
    assert_eq!(
        ::std::mem::align_of::<ClassCInnerCRTP>(),
        1usize,
        concat!("Alignment of ", stringify!(ClassCInnerCRTP))
    );
}
impl Default for ClassCInnerCRTP {
    fn default() -> Self {
        let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
#[test]
fn __bindgen_test_layout_ClassB_open0_ClassCInnerCRTP_ClassAInner_close0_instantiation(
) {
    assert_eq!(
        ::std::mem::size_of::<ClassB>(),
        1usize,
        concat!("Size of template specialization: ", stringify!(ClassB))
    );
    assert_eq!(
        ::std::mem::align_of::<ClassB>(),
        1usize,
        concat!("Alignment of template specialization: ", stringify!(ClassB))
    );
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct ClassAInner {
    pub x: *mut ClassCInnerA,
}
#[test]
fn bindgen_test_layout_ClassAInner() {
    const UNINIT: ::std::mem::MaybeUninit<ClassAInner> =
        ::std::mem::MaybeUninit::uninit();
    let ptr = UNINIT.as_ptr();
    assert_eq!(
        ::std::mem::size_of::<ClassAInner>(),
        8usize,
        concat!("Size of: ", stringify!(ClassAInner))
    );
    assert_eq!(
        ::std::mem::align_of::<ClassAInner>(),
        8usize,
        concat!("Alignment of ", stringify!(ClassAInner))
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).x) as usize - ptr as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(ClassAInner),
            "::",
            stringify!(x)
        )
    );
}
impl Default for ClassAInner {
    fn default() -> Self {
        let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct ClassCInnerA {
    pub member: *mut ClassCInnerB,
}
#[test]
fn bindgen_test_layout_ClassCInnerA() {
    const UNINIT: ::std::mem::MaybeUninit<ClassCInnerA> =
        ::std::mem::MaybeUninit::uninit();
    let ptr = UNINIT.as_ptr();
    assert_eq!(
        ::std::mem::size_of::<ClassCInnerA>(),
        8usize,
        concat!("Size of: ", stringify!(ClassCInnerA))
    );
    assert_eq!(
        ::std::mem::align_of::<ClassCInnerA>(),
        8usize,
        concat!("Alignment of ", stringify!(ClassCInnerA))
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).member) as usize - ptr as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(ClassCInnerA),
            "::",
            stringify!(member)
        )
    );
}
impl Default for ClassCInnerA {
    fn default() -> Self {
        let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct ClassCInnerB {
    pub cache: *mut ClassCInnerA,
}
#[test]
fn bindgen_test_layout_ClassCInnerB() {
    const UNINIT: ::std::mem::MaybeUninit<ClassCInnerB> =
        ::std::mem::MaybeUninit::uninit();
    let ptr = UNINIT.as_ptr();
    assert_eq!(
        ::std::mem::size_of::<ClassCInnerB>(),
        8usize,
        concat!("Size of: ", stringify!(ClassCInnerB))
    );
    assert_eq!(
        ::std::mem::align_of::<ClassCInnerB>(),
        8usize,
        concat!("Alignment of ", stringify!(ClassCInnerB))
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).cache) as usize - ptr as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(ClassCInnerB),
            "::",
            stringify!(cache)
        )
    );
}
impl Default for ClassCInnerB {
    fn default() -> Self {
        let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
