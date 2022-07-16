#![allow(
    dead_code,
    non_snake_case,
    non_camel_case_types,
    non_upper_case_globals
)]

#[repr(C)]
#[derive(Debug, Default, Copy, Clone)]
pub struct Calc {
    pub w: ::std::os::raw::c_int,
}
#[test]
fn bindgen_test_layout_Calc() {
    const UNINIT: ::std::mem::MaybeUninit<Calc> =
        ::std::mem::MaybeUninit::uninit();
    let ptr = UNINIT.as_ptr();
    assert_eq!(
        ::std::mem::size_of::<Calc>(),
        4usize,
        concat!("Size of: ", stringify!(Calc))
    );
    assert_eq!(
        ::std::mem::align_of::<Calc>(),
        4usize,
        concat!("Alignment of ", stringify!(Calc))
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).w) as usize - ptr as usize },
        0usize,
        concat!("Offset of field: ", stringify!(Calc), "::", stringify!(w))
    );
}
#[repr(C)]
#[derive(Debug, Default, Copy, Clone)]
pub struct Test {
    pub _address: u8,
}
#[repr(C)]
#[derive(Debug, Default, Copy, Clone)]
pub struct Test_Size {
    pub mWidth: Test_Size_Dimension,
    pub mHeight: Test_Size_Dimension,
}
#[repr(C)]
#[derive(Debug, Default, Copy, Clone)]
pub struct Test_Size_Dimension {
    pub _base: Calc,
}
#[test]
fn bindgen_test_layout_Test_Size_Dimension() {
    assert_eq!(
        ::std::mem::size_of::<Test_Size_Dimension>(),
        4usize,
        concat!("Size of: ", stringify!(Test_Size_Dimension))
    );
    assert_eq!(
        ::std::mem::align_of::<Test_Size_Dimension>(),
        4usize,
        concat!("Alignment of ", stringify!(Test_Size_Dimension))
    );
}
#[test]
fn bindgen_test_layout_Test_Size() {
    const UNINIT: ::std::mem::MaybeUninit<Test_Size> =
        ::std::mem::MaybeUninit::uninit();
    let ptr = UNINIT.as_ptr();
    assert_eq!(
        ::std::mem::size_of::<Test_Size>(),
        8usize,
        concat!("Size of: ", stringify!(Test_Size))
    );
    assert_eq!(
        ::std::mem::align_of::<Test_Size>(),
        4usize,
        concat!("Alignment of ", stringify!(Test_Size))
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).mWidth) as usize - ptr as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(Test_Size),
            "::",
            stringify!(mWidth)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).mHeight) as usize - ptr as usize },
        4usize,
        concat!(
            "Offset of field: ",
            stringify!(Test_Size),
            "::",
            stringify!(mHeight)
        )
    );
}
#[test]
fn bindgen_test_layout_Test() {
    assert_eq!(
        ::std::mem::size_of::<Test>(),
        1usize,
        concat!("Size of: ", stringify!(Test))
    );
    assert_eq!(
        ::std::mem::align_of::<Test>(),
        1usize,
        concat!("Alignment of ", stringify!(Test))
    );
}
