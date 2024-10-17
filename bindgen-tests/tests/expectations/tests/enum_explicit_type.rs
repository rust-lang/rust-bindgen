#![allow(dead_code, non_snake_case, non_camel_case_types, non_upper_case_globals)]
pub type Foo_ctype = ::std::os::raw::c_uchar;
pub const Foo_Bar: Foo_ctype = 0;
pub const Foo_Qux: Foo_ctype = 1;
#[repr(u8)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum Foo {
    Bar = 0,
    Qux = 1,
}
pub type Neg_ctype = ::std::os::raw::c_schar;
pub const Neg_MinusOne: Neg_ctype = -1;
pub const Neg_One: Neg_ctype = 1;
#[repr(i8)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum Neg {
    MinusOne = -1,
    One = 1,
}
pub type Bigger_ctype = ::std::os::raw::c_ushort;
pub const Bigger_Much: Bigger_ctype = 255;
pub const Bigger_Larger: Bigger_ctype = 256;
#[repr(u16)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum Bigger {
    Much = 255,
    Larger = 256,
}
pub type MuchLong_ctype = ::std::os::raw::c_long;
pub const MuchLong_MuchLow: MuchLong_ctype = -4294967296;
#[repr(i64)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum MuchLong {
    MuchLow = -4294967296,
}
pub type MuchLongLong_ctype = ::std::os::raw::c_longlong;
pub const MuchLongLong_I64_MIN: MuchLongLong_ctype = -9223372036854775808;
#[repr(i64)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum MuchLongLong {
    I64_MIN = -9223372036854775808,
}
pub type MuchULongLong_ctype = ::std::os::raw::c_ulonglong;
pub const MuchULongLong_MuchHigh: MuchULongLong_ctype = 4294967296;
#[repr(u64)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum MuchULongLong {
    MuchHigh = 4294967296,
}
pub type BoolEnumsAreFun_ctype = bool;
pub const BoolEnumsAreFun_Value: BoolEnumsAreFun_ctype = 1 != 0;
#[repr(u8)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum BoolEnumsAreFun {
    Value = 1,
}
pub type MyType = bool;
pub type BoolEnumsAreFun2_ctype = MyType;
pub const BoolEnumsAreFun2_Value2: BoolEnumsAreFun2_ctype = 1 != 0;
#[repr(u8)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum BoolEnumsAreFun2 {
    Value2 = 1,
}
pub const AnonymousVariantOne: _bindgen_ty_1 = _bindgen_ty_1::AnonymousVariantOne;
pub const AnonymousVariantTwo: _bindgen_ty_1 = _bindgen_ty_1::AnonymousVariantTwo;
pub type _bindgen_ty_1_ctype = ::std::os::raw::c_uchar;
pub const _bindgen_ty_1_AnonymousVariantOne: _bindgen_ty_1_ctype = 0;
pub const _bindgen_ty_1_AnonymousVariantTwo: _bindgen_ty_1_ctype = 1;
#[repr(u8)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum _bindgen_ty_1 {
    AnonymousVariantOne = 0,
    AnonymousVariantTwo = 1,
}
