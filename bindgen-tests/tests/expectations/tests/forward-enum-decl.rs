#![allow(dead_code, non_snake_case, non_camel_case_types, non_upper_case_globals)]
pub type CSSPseudoClassType_ctype = ::std::os::raw::c_int;
pub const CSSPseudoClassType_empty: CSSPseudoClassType_ctype = 0;
pub const CSSPseudoClassType_link: CSSPseudoClassType_ctype = 1;
#[repr(i32)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum CSSPseudoClassType {
    empty = 0,
    link = 1,
}
