#![allow(dead_code, non_snake_case, non_camel_case_types, non_upper_case_globals)]
#[link(wasm_import_module = "test-module")]
unsafe extern "C" {
    pub fn test_function();
}
