#![allow(
    dead_code,
    non_snake_case,
    non_camel_case_types,
    non_upper_case_globals
)]

pub const nsCSSPropertyID_eCSSProperty_COUNT_unexistingVariantValue:
    nsCSSPropertyID =
    nsCSSPropertyID::eCSSProperty_COUNT_unexistingVariantValue;
impl nsCSSPropertyID {
    pub const eCSSProperty_COUNT: nsCSSPropertyID =
        nsCSSPropertyID::eCSSPropertyAlias_aa;
}
#[repr(u32)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum nsCSSPropertyID {
    eCSSProperty_a = 0,
    eCSSProperty_b = 1,
    eCSSPropertyAlias_aa = 2,
    eCSSPropertyAlias_bb = 3,
    ///< <div rustbindgen constant></div>
    eCSSProperty_COUNT_unexistingVariantValue = 4,
}
