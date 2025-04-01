#![allow(dead_code, non_snake_case, non_camel_case_types, non_upper_case_globals)]
#[repr(u32)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum Plain {
    Plain1 = 0,
    Plain2 = 1,
    Plain3 = 2,
}
pub type TryFromRaw_ctype = ::std::os::raw::c_int;
impl TryFromRaw {
    pub const TFR1: TryFromRaw_ctype = -1;
    pub const TFR2: TryFromRaw_ctype = 5;
    pub const TFR3: TryFromRaw_ctype = 6;
}
#[repr(i32)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum TryFromRaw {
    TFR1 = -1,
    TFR2 = 5,
    TFR3 = 6,
}
pub struct TryFromRawError(TryFromRaw_ctype);
impl TryFromRawError {
    #[must_use]
    pub fn value(&self) -> TryFromRaw_ctype {
        self.0
    }
}
impl std::convert::TryFrom<TryFromRaw_ctype> for TryFromRaw {
    type Error = TryFromRawError;
    fn try_from(v: TryFromRaw_ctype) -> Result<Self, Self::Error> {
        match v {
            -1 => Ok(TryFromRaw::TFR1),
            5 => Ok(TryFromRaw::TFR2),
            6 => Ok(TryFromRaw::TFR3),
            _ => Err(TryFromRawError(v)),
        }
    }
}
pub type FromRawUnchecked_ctype = ::std::os::raw::c_uint;
impl FromRawUnchecked {
    pub const FRU1: FromRawUnchecked_ctype = 6;
    pub const FRU2: FromRawUnchecked_ctype = 10;
    pub const FRU3: FromRawUnchecked_ctype = 11;
}
#[repr(u32)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum FromRawUnchecked {
    FRU1 = 6,
    FRU2 = 10,
    FRU3 = 11,
}
impl FromRawUnchecked {
    const unsafe fn from_ctype_unchecked(v: FromRawUnchecked_ctype) -> Self {
        std::mem::transmute(v)
    }
}
impl Both {
    pub const Both3: Both = Both::Both1;
}
pub type Both_ctype = ::std::os::raw::c_int;
impl Both {
    pub const Both1: Both_ctype = 0;
    pub const Both2: Both_ctype = -1;
}
#[repr(i32)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum Both {
    Both1 = 0,
    Both2 = -1,
}
pub struct BothError(Both_ctype);
impl BothError {
    #[must_use]
    pub fn value(&self) -> Both_ctype {
        self.0
    }
}
impl std::convert::TryFrom<Both_ctype> for Both {
    type Error = BothError;
    fn try_from(v: Both_ctype) -> Result<Self, Self::Error> {
        match v {
            0 => Ok(Both::Both1),
            -1 => Ok(Both::Both2),
            _ => Err(BothError(v)),
        }
    }
}
impl Both {
    const unsafe fn from_ctype_unchecked(v: Both_ctype) -> Self {
        std::mem::transmute(v)
    }
}
#[repr(u32)]
#[non_exhaustive]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum NonExhaustive {
    Ex1 = 0,
    Ex2 = 1,
}
