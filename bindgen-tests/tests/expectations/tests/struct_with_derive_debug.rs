#![allow(dead_code, non_snake_case, non_camel_case_types, non_upper_case_globals)]
#[repr(C)]
#[derive(Debug, Default, Copy, Clone, Hash, PartialEq, Eq)]
pub struct LittleArray {
    pub a: [::std::os::raw::c_int; 32usize],
}
#[test]
fn bindgen_test_layout_LittleArray() {
    const UNINIT: ::std::mem::MaybeUninit<LittleArray> = ::std::mem::MaybeUninit::uninit();
    let ptr = UNINIT.as_ptr();
    assert_eq!(::std::mem::size_of::<LittleArray>(), 128usize, "Size of LittleArray");
    assert_eq!(
        ::std::mem::align_of::<LittleArray>(),
        4usize,
        "Alignment of LittleArray",
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).a) as usize - ptr as usize },
        0usize,
        "Offset of field: LittleArray::a",
    );
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct BigArray {
    pub a: [::std::os::raw::c_int; 33usize],
}
#[test]
fn bindgen_test_layout_BigArray() {
    const UNINIT: ::std::mem::MaybeUninit<BigArray> = ::std::mem::MaybeUninit::uninit();
    let ptr = UNINIT.as_ptr();
    assert_eq!(::std::mem::size_of::<BigArray>(), 132usize, "Size of BigArray");
    assert_eq!(::std::mem::align_of::<BigArray>(), 4usize, "Alignment of BigArray");
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).a) as usize - ptr as usize },
        0usize,
        "Offset of field: BigArray::a",
    );
}
impl Default for BigArray {
    fn default() -> Self {
        let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
#[repr(C)]
#[derive(Debug, Default, Copy, Clone, Hash, PartialEq, Eq)]
pub struct WithLittleArray {
    pub a: LittleArray,
}
#[test]
fn bindgen_test_layout_WithLittleArray() {
    const UNINIT: ::std::mem::MaybeUninit<WithLittleArray> = ::std::mem::MaybeUninit::uninit();
    let ptr = UNINIT.as_ptr();
    assert_eq!(
        ::std::mem::size_of::<WithLittleArray>(),
        128usize,
        "Size of WithLittleArray",
    );
    assert_eq!(
        ::std::mem::align_of::<WithLittleArray>(),
        4usize,
        "Alignment of WithLittleArray",
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).a) as usize - ptr as usize },
        0usize,
        "Offset of field: WithLittleArray::a",
    );
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct WithBigArray {
    pub a: BigArray,
}
#[test]
fn bindgen_test_layout_WithBigArray() {
    const UNINIT: ::std::mem::MaybeUninit<WithBigArray> = ::std::mem::MaybeUninit::uninit();
    let ptr = UNINIT.as_ptr();
    assert_eq!(::std::mem::size_of::<WithBigArray>(), 132usize, "Size of WithBigArray");
    assert_eq!(
        ::std::mem::align_of::<WithBigArray>(),
        4usize,
        "Alignment of WithBigArray",
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).a) as usize - ptr as usize },
        0usize,
        "Offset of field: WithBigArray::a",
    );
}
impl Default for WithBigArray {
    fn default() -> Self {
        let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
