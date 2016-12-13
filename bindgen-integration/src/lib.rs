mod bindings {
    include!(concat!(env!("OUT_DIR"), "/test.rs"));
}

use std::ffi::CStr;

#[test]
fn test_static_method() {
    let c_str = unsafe { bindings::Test::name() };
    let name = unsafe {
        CStr::from_ptr(c_str).to_string_lossy().into_owned()
    };
    assert_eq!(name, "Test", "Calling a static C++ method works!");
}

#[test]
fn test_constructor() {
    let test = unsafe { bindings::Test::new(5) };
    assert_eq!(test.m_int, 5);
    assert_eq!(test.m_double, 0.0);
}

#[test]
fn test_overload() {
    let test = unsafe { bindings::Test::new1(5.0) };
    assert_eq!(test.m_int, 0);
    assert_eq!(test.m_double, 5.0);
}
