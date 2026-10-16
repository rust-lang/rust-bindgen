#![allow(dead_code, non_snake_case, non_camel_case_types, non_upper_case_globals)]
/// <div rustbindgen replaces="PASS"></div>
///
/// Should see PASS below.
pub const OGRErr_PASS: OGRErr = 0;
/// <div rustbindgen replaces="OGRERR_NONE"></div>
///
/// Should see OGRERR_NONE instead of CUSTOM_OGRERR_NONE below.
pub const OGRErr_OGRERR_NONE: OGRErr = 1;
/// <div rustbindgen replaces="OGRErr"></div>
pub type OGRErr = ::std::os::raw::c_uint;
