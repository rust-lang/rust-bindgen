#![allow(
    dead_code,
    non_snake_case,
    non_camel_case_types,
    non_upper_case_globals
)]
#![cfg(not(target_os = "windows"))]

pub const Foo_Bar: Foo = 0;
pub const Foo_Qux: Foo = 1;
pub type Foo = ::std::os::raw::c_uchar;
pub const Neg_MinusOne: Neg = -1;
pub const Neg_One: Neg = 1;
pub type Neg = ::std::os::raw::c_schar;
pub const Bigger_Much: Bigger = 255;
pub const Bigger_Larger: Bigger = 256;
pub type Bigger = ::std::os::raw::c_ushort;
pub const MuchLong_MuchLow: MuchLong = -4294967296;
pub type MuchLong = ::std::os::raw::c_long;
pub const MuchLongLong_I64_MIN: MuchLongLong = -9223372036854775808;
pub type MuchLongLong = ::std::os::raw::c_longlong;
pub const MuchULongLong_MuchHigh: MuchULongLong = 4294967296;
pub type MuchULongLong = ::std::os::raw::c_ulonglong;
pub const BoolEnumsAreFun_Value: BoolEnumsAreFun = true;
pub type BoolEnumsAreFun = bool;
pub type MyType = bool;
pub const BoolEnumsAreFun2_Value2: BoolEnumsAreFun2 = true;
pub type BoolEnumsAreFun2 = MyType;
pub const AnonymousVariantOne: _bindgen_ty_1 = 0;
pub const AnonymousVariantTwo: _bindgen_ty_1 = 1;
pub type _bindgen_ty_1 = ::std::os::raw::c_uchar;
