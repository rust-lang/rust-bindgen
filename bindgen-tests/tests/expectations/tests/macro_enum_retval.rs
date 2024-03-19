#![allow(dead_code, non_snake_case, non_camel_case_types, non_upper_case_globals)]
pub const truth_wrong: truth = 0;
pub const truth_right: truth = 1;
pub type truth = ::std::os::raw::c_uint;
#[allow(non_snake_case, unused_mut, unsafe_code)]
#[inline(always)]
pub unsafe extern "C" fn get_wrong() -> truth {
    truth_wrong
}
