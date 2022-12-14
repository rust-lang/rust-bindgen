#![allow(
    dead_code,
    non_snake_case,
    non_camel_case_types,
    non_upper_case_globals
)]

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct mozilla_MarkerSchema {
    _unused: [u8; 0],
}
extern "C" {
    pub fn gecko_profiler_marker_schema_set_chart_label(
        schema: *mut mozilla_MarkerSchema,
        label: *const ::std::os::raw::c_char,
        len: usize,
    );
}
