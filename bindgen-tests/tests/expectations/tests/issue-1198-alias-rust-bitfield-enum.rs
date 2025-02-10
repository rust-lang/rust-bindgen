#![allow(dead_code, non_snake_case, non_camel_case_types, non_upper_case_globals)]
impl MyDupeEnum {
    pub const A: MyDupeEnum = MyDupeEnum(0);
    pub const A_alias: MyDupeEnum = MyDupeEnum(0);
    pub const B: MyDupeEnum = MyDupeEnum(1);
}
impl ::std::ops::BitOr<MyDupeEnum> for MyDupeEnum {
    type Output = Self;
    #[inline]
    fn bitor(self, other: Self) -> Self {
        MyDupeEnum(self.0 | other.0)
    }
}
impl ::std::ops::BitOrAssign for MyDupeEnum {
    #[inline]
    fn bitor_assign(&mut self, rhs: MyDupeEnum) {
        self.0 |= rhs.0;
    }
}
impl ::std::ops::BitAnd<MyDupeEnum> for MyDupeEnum {
    type Output = Self;
    #[inline]
    fn bitand(self, other: Self) -> Self {
        MyDupeEnum(self.0 & other.0)
    }
}
impl ::std::ops::BitAndAssign for MyDupeEnum {
    #[inline]
    fn bitand_assign(&mut self, rhs: MyDupeEnum) {
        self.0 &= rhs.0;
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct MyDupeEnum(pub ::std::os::raw::c_uint);
impl MyOtherDupeEnum {
    pub const C: MyOtherDupeEnum = MyOtherDupeEnum(0);
    pub const C_alias: MyOtherDupeEnum = MyOtherDupeEnum(0);
    pub const D: MyOtherDupeEnum = MyOtherDupeEnum(1);
}
impl ::std::ops::BitOr<MyOtherDupeEnum> for MyOtherDupeEnum {
    type Output = Self;
    #[inline]
    fn bitor(self, other: Self) -> Self {
        MyOtherDupeEnum(self.0 | other.0)
    }
}
impl ::std::ops::BitOrAssign for MyOtherDupeEnum {
    #[inline]
    fn bitor_assign(&mut self, rhs: MyOtherDupeEnum) {
        self.0 |= rhs.0;
    }
}
impl ::std::ops::BitAnd<MyOtherDupeEnum> for MyOtherDupeEnum {
    type Output = Self;
    #[inline]
    fn bitand(self, other: Self) -> Self {
        MyOtherDupeEnum(self.0 & other.0)
    }
}
impl ::std::ops::BitAndAssign for MyOtherDupeEnum {
    #[inline]
    fn bitand_assign(&mut self, rhs: MyOtherDupeEnum) {
        self.0 &= rhs.0;
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct MyOtherDupeEnum(pub ::std::os::raw::c_uint);
