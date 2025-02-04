#![allow(dead_code, non_snake_case, non_camel_case_types, non_upper_case_globals)]
impl B {
    /// Document field with three slashes
    pub const VAR_A: B = B(0);
    /// Document field with preceding star
    pub const VAR_B: B = B(1);
    /// Document field with preceding exclamation
    pub const VAR_C: B = B(2);
    ///< Document field with following star
    pub const VAR_D: B = B(3);
    ///< Document field with following exclamation
    pub const VAR_E: B = B(4);
    /** Document field with preceding star, with a loong long multiline
 comment.

 Very interesting documentation, definitely.*/
    pub const VAR_F: B = B(5);
}
impl ::std::ops::BitOr<B> for B {
    type Output = Self;
    #[inline]
    fn bitor(self, other: Self) -> Self {
        B(self.0 | other.0)
    }
}
impl ::std::ops::BitOrAssign for B {
    #[inline]
    fn bitor_assign(&mut self, rhs: B) {
        self.0 |= rhs.0;
    }
}
impl ::std::ops::BitAnd<B> for B {
    type Output = Self;
    #[inline]
    fn bitand(self, other: Self) -> Self {
        B(self.0 & other.0)
    }
}
impl ::std::ops::BitAndAssign for B {
    #[inline]
    fn bitand_assign(&mut self, rhs: B) {
        self.0 &= rhs.0;
    }
}
#[repr(transparent)]
/// Document enum
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct B(pub ::std::os::raw::c_uint);
