#![allow(dead_code, non_snake_case, non_camel_case_types, non_upper_case_globals)]
#[cfg_attr(windows, link(name = "foolib", kind = "raw-dylib"))]
unsafe extern "C" {
    pub fn test_function();
}
