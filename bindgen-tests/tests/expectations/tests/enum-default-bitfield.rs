#![allow(dead_code, non_snake_case, non_camel_case_types, non_upper_case_globals)]
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct foo {
    pub member: foo__bindgen_ty_1,
}
pub const foo_FOO_A: foo__bindgen_ty_1 = foo__bindgen_ty_1(0);
pub const foo_FOO_B: foo__bindgen_ty_1 = foo__bindgen_ty_1(1);
impl ::std::ops::BitOr<foo__bindgen_ty_1> for foo__bindgen_ty_1 {
    type Output = Self;
    #[inline]
    fn bitor(self, other: Self) -> Self {
        foo__bindgen_ty_1(self.0 | other.0)
    }
}
impl ::std::ops::BitOrAssign for foo__bindgen_ty_1 {
    #[inline]
    fn bitor_assign(&mut self, rhs: foo__bindgen_ty_1) {
        self.0 |= rhs.0;
    }
}
impl ::std::ops::BitAnd<foo__bindgen_ty_1> for foo__bindgen_ty_1 {
    type Output = Self;
    #[inline]
    fn bitand(self, other: Self) -> Self {
        foo__bindgen_ty_1(self.0 & other.0)
    }
}
impl ::std::ops::BitAndAssign for foo__bindgen_ty_1 {
    #[inline]
    fn bitand_assign(&mut self, rhs: foo__bindgen_ty_1) {
        self.0 &= rhs.0;
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct foo__bindgen_ty_1(pub ::std::os::raw::c_uint);
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of foo"][::std::mem::size_of::<foo>() - 4usize];
    ["Alignment of foo"][::std::mem::align_of::<foo>() - 4usize];
    ["Offset of field: foo::member"][::std::mem::offset_of!(foo, member) - 0usize];
};
impl Default for foo {
    fn default() -> Self {
        let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
impl Foo {
    pub const Bar: Foo = Foo(0);
    pub const Qux: Foo = Foo(1);
}
impl ::std::ops::BitOr<Foo> for Foo {
    type Output = Self;
    #[inline]
    fn bitor(self, other: Self) -> Self {
        Foo(self.0 | other.0)
    }
}
impl ::std::ops::BitOrAssign for Foo {
    #[inline]
    fn bitor_assign(&mut self, rhs: Foo) {
        self.0 |= rhs.0;
    }
}
impl ::std::ops::BitAnd<Foo> for Foo {
    type Output = Self;
    #[inline]
    fn bitand(self, other: Self) -> Self {
        Foo(self.0 & other.0)
    }
}
impl ::std::ops::BitAndAssign for Foo {
    #[inline]
    fn bitand_assign(&mut self, rhs: Foo) {
        self.0 &= rhs.0;
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct Foo(pub ::std::os::raw::c_uint);
pub mod Neg {
    #[allow(unused_imports)]
    use super::*;
    pub type Type = ::std::os::raw::c_int;
    pub const MinusOne: Type = -1;
    pub const One: Type = 1;
}
impl NoDebug {
    pub const NoDebug1: NoDebug = NoDebug(0);
    pub const NoDebug2: NoDebug = NoDebug(1);
}
impl ::std::ops::BitOr<NoDebug> for NoDebug {
    type Output = Self;
    #[inline]
    fn bitor(self, other: Self) -> Self {
        NoDebug(self.0 | other.0)
    }
}
impl ::std::ops::BitOrAssign for NoDebug {
    #[inline]
    fn bitor_assign(&mut self, rhs: NoDebug) {
        self.0 |= rhs.0;
    }
}
impl ::std::ops::BitAnd<NoDebug> for NoDebug {
    type Output = Self;
    #[inline]
    fn bitand(self, other: Self) -> Self {
        NoDebug(self.0 & other.0)
    }
}
impl ::std::ops::BitAndAssign for NoDebug {
    #[inline]
    fn bitand_assign(&mut self, rhs: NoDebug) {
        self.0 &= rhs.0;
    }
}
#[repr(transparent)]
/// <div rustbindgen nodebug></div>
#[derive(Copy, Clone, Hash, PartialEq, Eq)]
pub struct NoDebug(pub ::std::os::raw::c_uint);
impl Debug {
    pub const Debug1: Debug = Debug(0);
    pub const Debug2: Debug = Debug(1);
}
impl ::std::ops::BitOr<Debug> for Debug {
    type Output = Self;
    #[inline]
    fn bitor(self, other: Self) -> Self {
        Debug(self.0 | other.0)
    }
}
impl ::std::ops::BitOrAssign for Debug {
    #[inline]
    fn bitor_assign(&mut self, rhs: Debug) {
        self.0 |= rhs.0;
    }
}
impl ::std::ops::BitAnd<Debug> for Debug {
    type Output = Self;
    #[inline]
    fn bitand(self, other: Self) -> Self {
        Debug(self.0 & other.0)
    }
}
impl ::std::ops::BitAndAssign for Debug {
    #[inline]
    fn bitand_assign(&mut self, rhs: Debug) {
        self.0 &= rhs.0;
    }
}
#[repr(transparent)]
/// <div rustbindgen derive="Debug"></div>
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct Debug(pub ::std::os::raw::c_uint);
impl should_be_u8 {
    pub const FirstU8: should_be_u8 = should_be_u8(0);
    pub const SecondU8: should_be_u8 = should_be_u8(1);
}
impl ::std::ops::BitOr<should_be_u8> for should_be_u8 {
    type Output = Self;
    #[inline]
    fn bitor(self, other: Self) -> Self {
        should_be_u8(self.0 | other.0)
    }
}
impl ::std::ops::BitOrAssign for should_be_u8 {
    #[inline]
    fn bitor_assign(&mut self, rhs: should_be_u8) {
        self.0 |= rhs.0;
    }
}
impl ::std::ops::BitAnd<should_be_u8> for should_be_u8 {
    type Output = Self;
    #[inline]
    fn bitand(self, other: Self) -> Self {
        should_be_u8(self.0 & other.0)
    }
}
impl ::std::ops::BitAndAssign for should_be_u8 {
    #[inline]
    fn bitand_assign(&mut self, rhs: should_be_u8) {
        self.0 &= rhs.0;
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct should_be_u8(pub u8);
impl should_be_u16 {
    pub const FirstU16: should_be_u16 = should_be_u16(0);
    pub const SecondU16: should_be_u16 = should_be_u16(1);
}
impl ::std::ops::BitOr<should_be_u16> for should_be_u16 {
    type Output = Self;
    #[inline]
    fn bitor(self, other: Self) -> Self {
        should_be_u16(self.0 | other.0)
    }
}
impl ::std::ops::BitOrAssign for should_be_u16 {
    #[inline]
    fn bitor_assign(&mut self, rhs: should_be_u16) {
        self.0 |= rhs.0;
    }
}
impl ::std::ops::BitAnd<should_be_u16> for should_be_u16 {
    type Output = Self;
    #[inline]
    fn bitand(self, other: Self) -> Self {
        should_be_u16(self.0 & other.0)
    }
}
impl ::std::ops::BitAndAssign for should_be_u16 {
    #[inline]
    fn bitand_assign(&mut self, rhs: should_be_u16) {
        self.0 &= rhs.0;
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct should_be_u16(pub u16);
impl should_be_i32 {
    pub const FirstI32: should_be_i32 = should_be_i32(0);
    pub const SecondI32: should_be_i32 = should_be_i32(1);
}
impl ::std::ops::BitOr<should_be_i32> for should_be_i32 {
    type Output = Self;
    #[inline]
    fn bitor(self, other: Self) -> Self {
        should_be_i32(self.0 | other.0)
    }
}
impl ::std::ops::BitOrAssign for should_be_i32 {
    #[inline]
    fn bitor_assign(&mut self, rhs: should_be_i32) {
        self.0 |= rhs.0;
    }
}
impl ::std::ops::BitAnd<should_be_i32> for should_be_i32 {
    type Output = Self;
    #[inline]
    fn bitand(self, other: Self) -> Self {
        should_be_i32(self.0 & other.0)
    }
}
impl ::std::ops::BitAndAssign for should_be_i32 {
    #[inline]
    fn bitand_assign(&mut self, rhs: should_be_i32) {
        self.0 &= rhs.0;
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct should_be_i32(pub i32);
