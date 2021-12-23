#![allow(
    dead_code,
    non_snake_case,
    non_camel_case_types,
    non_upper_case_globals
)]

#[repr(C)]
#[derive(Debug, Default, Copy, Clone, Hash, PartialEq, Eq)]
pub struct DataType {
    pub _address: u8,
}
pub type DataType_value_type<_Tp> = _Tp;
pub type DataType_work_type<_Tp> = DataType_value_type<_Tp>;
pub type DataType_channel_type<_Tp> = DataType_value_type<_Tp>;
pub type DataType_vec_type<_Tp> = DataType_value_type<_Tp>;
pub const DataType_generic_type: DataType__bindgen_ty_1 =
    DataType__bindgen_ty_1::generic_type;
pub const DataType_depth: DataType__bindgen_ty_1 =
    DataType__bindgen_ty_1::generic_type;
pub const DataType_channels: DataType__bindgen_ty_1 =
    DataType__bindgen_ty_1::generic_type;
pub const DataType_fmt: DataType__bindgen_ty_1 =
    DataType__bindgen_ty_1::generic_type;
pub const DataType_type_: DataType__bindgen_ty_1 =
    DataType__bindgen_ty_1::generic_type;
#[repr(i32)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum DataType__bindgen_ty_1 {
    generic_type = 0,
}
#[repr(C)]
#[derive(Debug, Default, Copy, Clone, Hash, PartialEq, Eq)]
pub struct Foo {
    pub _address: u8,
}
pub const Foo_Bar: Foo__bindgen_ty_1 = Foo__bindgen_ty_1::Bar;
pub const Foo_Baz: Foo__bindgen_ty_1 = Foo__bindgen_ty_1::Bar;
#[repr(u32)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum Foo__bindgen_ty_1 {
    Bar = 0,
}
#[test]
fn bindgen_test_layout_Foo() {
    assert_eq!(
        ::std::mem::size_of::<Foo>(),
        1usize,
        concat!("Size of: ", stringify!(Foo))
    );
    assert_eq!(
        ::std::mem::align_of::<Foo>(),
        1usize,
        concat!("Alignment of ", stringify!(Foo))
    );
}
