#![allow(
    dead_code,
    non_snake_case,
    non_camel_case_types,
    non_upper_case_globals
)]

#[derive(PartialEq, Copy, Clone, Hash, Debug, Default)]
#[repr(C)]
pub struct __BindgenComplex<T> {
    pub re: T,
    pub im: T,
}
#[repr(C)]
#[derive(Debug, Default, Copy, Clone, PartialEq)]
pub struct TestDouble {
    pub mMember: __BindgenComplex<f64>,
}
#[test]
fn bindgen_test_layout_TestDouble() {
    assert_eq!(
        ::std::mem::size_of::<TestDouble>(),
        16usize,
        concat!("Size of: ", stringify!(TestDouble))
    );
    assert_eq!(
        ::std::mem::align_of::<TestDouble>(),
        8usize,
        concat!("Alignment of ", stringify!(TestDouble))
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<TestDouble>())).mMember as *const _ as usize
        },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(TestDouble),
            "::",
            stringify!(mMember)
        )
    );
}
#[repr(C)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct TestDoublePtr {
    pub mMember: *mut __BindgenComplex<f64>,
}
#[test]
fn bindgen_test_layout_TestDoublePtr() {
    assert_eq!(
        ::std::mem::size_of::<TestDoublePtr>(),
        8usize,
        concat!("Size of: ", stringify!(TestDoublePtr))
    );
    assert_eq!(
        ::std::mem::align_of::<TestDoublePtr>(),
        8usize,
        concat!("Alignment of ", stringify!(TestDoublePtr))
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<TestDoublePtr>())).mMember as *const _
                as usize
        },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(TestDoublePtr),
            "::",
            stringify!(mMember)
        )
    );
}
impl Default for TestDoublePtr {
    fn default() -> Self {
        let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
#[repr(C)]
#[derive(Debug, Default, Copy, Clone, PartialEq)]
pub struct TestFloat {
    pub mMember: __BindgenComplex<f32>,
}
#[test]
fn bindgen_test_layout_TestFloat() {
    assert_eq!(
        ::std::mem::size_of::<TestFloat>(),
        8usize,
        concat!("Size of: ", stringify!(TestFloat))
    );
    assert_eq!(
        ::std::mem::align_of::<TestFloat>(),
        4usize,
        concat!("Alignment of ", stringify!(TestFloat))
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<TestFloat>())).mMember as *const _ as usize
        },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(TestFloat),
            "::",
            stringify!(mMember)
        )
    );
}
#[repr(C)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct TestFloatPtr {
    pub mMember: *mut __BindgenComplex<f32>,
}
#[test]
fn bindgen_test_layout_TestFloatPtr() {
    assert_eq!(
        ::std::mem::size_of::<TestFloatPtr>(),
        8usize,
        concat!("Size of: ", stringify!(TestFloatPtr))
    );
    assert_eq!(
        ::std::mem::align_of::<TestFloatPtr>(),
        8usize,
        concat!("Alignment of ", stringify!(TestFloatPtr))
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<TestFloatPtr>())).mMember as *const _
                as usize
        },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(TestFloatPtr),
            "::",
            stringify!(mMember)
        )
    );
}
impl Default for TestFloatPtr {
    fn default() -> Self {
        let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
