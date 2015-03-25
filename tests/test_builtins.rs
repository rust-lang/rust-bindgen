use bindgen;

#[test]
fn test_builtin_va_list() {
	let bindings = bindgen::builder().header("tests/headers/builtin_va_list.h")
				       .emit_builtins().generate().unwrap().to_string();
    println!("{}", bindings);
    assert!(bindings.contains("__builtin_va_list"));
}

