use bindgen;

#[test]
fn unsigned() {
    let bindings = bindgen::Builder::new("tests/headers/unsigned.h")
        .generate()
        .unwrap()
        .to_string();

    assert!(bindings.contains("pub type size_t = usize;"));
    assert!(bindings.contains("pub type uintptr_t = usize;"));
    assert!(bindings.contains("pub type uint8_t = u8;"));
    assert!(bindings.contains("pub type uint16_t = u16;"));
    assert!(bindings.contains("pub type uint32_t = u32;"));
    assert!(bindings.contains("pub type uint64_t = u64;"));
}

#[test]
fn signed() {
    let bindings = bindgen::Builder::new("tests/headers/signed.h")
        .generate()
        .unwrap()
        .to_string();

    assert!(bindings.contains("pub type ptrdiff_t = isize;"));
    assert!(bindings.contains("pub type intptr_t = isize;"));
    assert!(bindings.contains("pub type ssize_t = isize;"));
    assert!(bindings.contains("pub type int8_t = i8;"));
    assert!(bindings.contains("pub type int16_t = i16;"));
    assert!(bindings.contains("pub type int32_t = i32;"));
    assert!(bindings.contains("pub type int64_t = i64;"));
}

#[test]
fn floats() {
    let bindings = bindgen::Builder::new("tests/headers/floats.h")
        .generate()
        .unwrap()
        .to_string();

    assert!(bindings.contains("pub type float = f32;"));
    assert!(bindings.contains("pub type double = f64;"));
}
