#[test]
fn test_builtin_va_list() {
    #[allow(non_camel_case_types)]
    mod ffi { bindgen!("headers/builtin_va_list.h", emit_builtins = true); }
    // Should test for more than compilation.
}

