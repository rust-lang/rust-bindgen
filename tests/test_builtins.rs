#[test]
fn test_builtin_va_list() {
    #[allow(dead_code, non_camel_case_types, raw_pointer_derive)]
    mod ffi { bindgen!("headers/builtin_va_list.h", emit_builtins = true); }
    // Should test for more than compilation.
}

