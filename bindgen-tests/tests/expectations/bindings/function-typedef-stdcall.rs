#![allow(
    dead_code,
    non_snake_case,
    non_camel_case_types,
    non_upper_case_globals
)]

pub type PFN_VIGEM_X360_NOTIFICATION = ::std::option::Option<
    unsafe extern "C" fn(
        arg1: *mut ::std::os::raw::c_void,
        arg2: *mut ::std::os::raw::c_void,
        arg3: ::std::os::raw::c_uchar,
        arg4: ::std::os::raw::c_uchar,
        arg5: ::std::os::raw::c_uchar,
        arg6: *mut ::std::os::raw::c_void,
    ),
>;
