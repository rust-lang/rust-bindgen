#![allow(dead_code, non_snake_case, non_camel_case_types, non_upper_case_globals)]
#[repr(C)]
#[derive(Debug, Default, Copy, Clone)]
pub struct Calc {
    pub w: ::std::os::raw::c_int,
}
const _: () = {
    ["Size of Calc"][::std::mem::size_of::<Calc>() - 4usize];
    ["Alignment of Calc"][::std::mem::align_of::<Calc>() - 4usize];
    ["Offset of field: Calc::w"][::std::mem::offset_of!(Calc, w) - 0usize];
};
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
const _: () = {
    [
        "Size of Test_Size_Dimension",
    ][::std::mem::size_of::<Test_Size_Dimension>() - 4usize];
    [
        "Alignment of Test_Size_Dimension",
    ][::std::mem::align_of::<Test_Size_Dimension>() - 4usize];
};
const _: () = {
    ["Size of Test_Size"][::std::mem::size_of::<Test_Size>() - 8usize];
    ["Alignment of Test_Size"][::std::mem::align_of::<Test_Size>() - 4usize];
    [
        "Offset of field: Test_Size::mWidth",
    ][::std::mem::offset_of!(Test_Size, mWidth) - 0usize];
    [
        "Offset of field: Test_Size::mHeight",
    ][::std::mem::offset_of!(Test_Size, mHeight) - 4usize];
};
const _: () = {
    ["Size of Test"][::std::mem::size_of::<Test>() - 1usize];
    ["Alignment of Test"][::std::mem::align_of::<Test>() - 1usize];
};
