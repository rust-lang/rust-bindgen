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
        {
            let struct_instance = unsafe { std::mem::zeroed::<Calc>() };
            let struct_ptr = &struct_instance as *const Calc;
            let field_ptr = std::ptr::addr_of!(struct_instance.w);
            let struct_address = struct_ptr as usize;
            let field_address = field_ptr as usize;
            std::mem::forget(struct_instance);
            field_address.checked_sub(struct_address).unwrap()
        },
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
        {
            let struct_instance = unsafe { std::mem::zeroed::<Test_Size>() };
            let struct_ptr = &struct_instance as *const Test_Size;
            let field_ptr = std::ptr::addr_of!(struct_instance.mWidth);
            let struct_address = struct_ptr as usize;
            let field_address = field_ptr as usize;
            std::mem::forget(struct_instance);
            field_address.checked_sub(struct_address).unwrap()
        },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(Test_Size),
            "::",
            stringify!(mWidth)
        )
    );
    assert_eq!(
        {
            let struct_instance = unsafe { std::mem::zeroed::<Test_Size>() };
            let struct_ptr = &struct_instance as *const Test_Size;
            let field_ptr = std::ptr::addr_of!(struct_instance.mHeight);
            let struct_address = struct_ptr as usize;
            let field_address = field_ptr as usize;
            std::mem::forget(struct_instance);
            field_address.checked_sub(struct_address).unwrap()
        },
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
